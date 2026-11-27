use std::env;

use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use validator::Validate;

use crate::entities::HasHashId;
use crate::entities::auth::role::Roles;
use crate::entities::list::playlist;
use crate::enums::list::playlistvisibility::PlaylistVisibility;
use crate::events::list::playlist_created::PlaylistCreated;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::has_any_role;
use crate::{AppError, events};

#[derive(InputObject, Validate)]
#[graphql(name = "CreatePlaylistInput")]
pub struct CreatePlaylistActionParameters {
    #[validate(length(
        min = 1,
        max = 192,
        message = "The name must be between 1 and 192 characters."
    ))]
    pub name: String,

    pub visibility: PlaylistVisibility,

    #[validate(length(
        min = 1,
        max = 1000,
        message = "The description must be between 1 and 1000 characters."
    ))]
    pub description: Option<String>,

    #[graphql(skip)]
    pub user_id: u64,
}

pub struct CreatePlaylistAction;

impl CreatePlaylistAction {
    pub async fn create(
        ctx: &AppContext,
        params: CreatePlaylistActionParameters,
        user: &CurrentUser,
    ) -> Result<playlist::Model, AppError> {
        if let Err(errors) = params.validate() {
            return Err(AppError::Validation(errors));
        }

        let count = playlist::Entity::find()
            .filter(playlist::Column::UserId.eq(params.user_id))
            .count(&ctx.db)
            .await?;

        let max_playlists: u64 = env::var("USER_MAX_PLAYLISTS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1000);

        if count >= max_playlists && !has_any_role(&user.roles, vec![Roles::Admin, Roles::Patron]) {
            return Err(AppError::ForbiddenWithMessage(
                "You have reached the maximum number of playlists".to_string(),
            ));
        }

        let playlist = playlist::ActiveModel {
            name: Set(params.name.trim().to_string()),
            description: Set(params.description.map(|d| d.trim().to_string())),
            visibility: Set(params.visibility),
            user_id: Set(params.user_id),
            ..Default::default()
        };

        let mut playlist = playlist.insert(&ctx.db).await?;

        playlist.update_hashid(&ctx.db).await?;

        events::dispatch(
            ctx,
            PlaylistCreated {
                playlist: playlist.clone(),
                user: Some(user.clone()),
            },
        )
        .await;

        Ok(playlist)
    }
}
