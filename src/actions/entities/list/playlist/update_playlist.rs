use anyhow::{Context, Result};
use async_graphql::{InputObject, MaybeUndefined};
use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use validator::Validate;

use crate::AppError;
use crate::actions::entities::{nullable_string, optional, optional_string};
use crate::entities::list::playlist;
use crate::enums::list::playlistvisibility::PlaylistVisibility;
use crate::initializers::typesense::typesense;
use crate::rules::*;
use crate::typesense::documents::playlist_document::{PlaylistDocument, build_playlist_documents};

#[derive(InputObject, Validate)]
#[graphql(name = "UpdatePlaylistInput")]
pub struct UpdatePlaylistActionParameters {
    #[validate(length(
        min = 1,
        max = 192,
        message = "The name must be between 1 and 192 characters."
    ))]
    pub name: Option<String>,

    pub visibility: Option<PlaylistVisibility>,

    #[validate(custom(
        function = "string::validate_length::<1, 1000>",
        message = "The description must be between 1 and 1000 characters."
    ))]
    pub description: MaybeUndefined<String>,
}

pub struct UpdatePlaylistAction;

impl UpdatePlaylistAction {
    pub async fn update(
        ctx: &AppContext,
        playlist: playlist::Model,
        params: UpdatePlaylistActionParameters,
    ) -> Result<playlist::Model, AppError> {
        if let Err(errors) = params.validate() {
            return Err(AppError::Validation(errors));
        }

        let playlist = playlist::ActiveModel {
            id: Set(playlist.id),
            name: optional_string(params.name),
            description: nullable_string(params.description),
            visibility: optional(params.visibility),
            ..Default::default()
        };

        let playlist = playlist.update(&ctx.db).await?;

        Self::update_search(&ctx.db, &playlist).await?;

        Ok(playlist)
    }

    async fn update_search(
        db: &DatabaseConnection,
        playlist: &playlist::Model,
    ) -> Result<(), AppError> {
        if playlist.visibility != PlaylistVisibility::Public {
            return Ok(());
        }

        let document = build_playlist_documents(vec![playlist.clone()], db)
            .await
            .map_err(AppError::internal)?
            .into_iter()
            .next()
            .context("Failed")
            .map_err(AppError::internal)?;

        let typesense = typesense();

        typesense
            .collection::<PlaylistDocument>()
            .documents()
            .upsert(&document, None)
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
