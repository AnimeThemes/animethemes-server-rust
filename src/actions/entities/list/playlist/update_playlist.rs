use anyhow::{Context, Result};
use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use validator::Validate;

use crate::AppError;
use crate::entities::list::playlist;
use crate::enums::list::playlistvisibility::PlaylistVisibility;
use crate::typesense::client::typesense;
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

    #[validate(length(
        min = 0,
        max = 1000,
        message = "The description must be between 1 and 1000 characters."
    ))]
    pub description: Option<String>,
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

        let mut playlist = playlist::ActiveModel {
            id: Set(playlist.id),
            ..Default::default()
        };

        if let Some(name) = params.name {
            playlist.name = Set(name);
        }

        if let Some(description) = params.description {
            if description.trim().is_empty() {
                playlist.description = Set(None);
            } else {
                playlist.description = Set(Some(description));
            }
        }

        if let Some(visibility) = params.visibility {
            playlist.visibility = Set(visibility);
        }

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
