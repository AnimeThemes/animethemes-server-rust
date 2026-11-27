use async_graphql::{InputObject, MaybeUndefined};
use loco_rs::app::AppContext;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use validator::Validate;

use crate::actions::entities::{nullable_string, optional, optional_string};
use crate::entities::list::playlist;
use crate::enums::list::playlistvisibility::PlaylistVisibility;
use crate::events::list::playlist_updated::PlaylistUpdated;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::*;
use crate::{AppError, events};

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
        user: &CurrentUser,
    ) -> Result<playlist::Model, AppError> {
        if let Err(errors) = params.validate() {
            return Err(AppError::Validation(errors));
        }

        let old_playlist = playlist.clone();

        let playlist = playlist::ActiveModel {
            id: Set(playlist.id),
            name: optional_string(params.name),
            description: nullable_string(params.description),
            visibility: optional(params.visibility),
            ..Default::default()
        };

        let playlist = playlist.update(&ctx.db).await?;

        events::dispatch(
            ctx,
            PlaylistUpdated {
                old_playlist: old_playlist,
                new_playlist: playlist.clone(),
                user: Some(user.clone()),
            },
        )
        .await;

        Ok(playlist)
    }
}
