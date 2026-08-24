use crate::AppError;
use crate::actions::entities::list::playlist::create_playlist::{
    CreatePlaylistAction, CreatePlaylistActionParameters,
};
use crate::actions::entities::list::playlist::delete_playlist::DeletePlaylistAction;
use crate::actions::entities::list::playlist::update_playlist::{
    UpdatePlaylistAction, UpdatePlaylistActionParameters,
};
use crate::entities::list::playlist;
use crate::enums::list::playlistvisibility::PlaylistVisibility;
use async_graphql::{Context, Error, InputObject, Object, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::graphql::types::list::playlist::Playlist;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::list::playlist::PlaylistPolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(InputObject)]
struct CreatePlaylistInput {
    #[graphql(validator(min_length = 1, max_length = 192))]
    name: String,
    #[graphql(validator(min_length = 1, max_length = 1000))]
    description: Option<String>,
    visibility: PlaylistVisibility,
}

#[derive(InputObject)]
struct UpdatePlaylistInput {
    #[graphql(validator(min_length = 1, max_length = 192))]
    name: Option<String>,
    #[graphql(validator(min_length = 1, max_length = 1000))]
    description: Option<String>,
    visibility: Option<PlaylistVisibility>,
}

#[derive(Default)]
pub struct PlaylistMutation;

#[Object]
impl PlaylistMutation {
    async fn create_playlist(
        &self,
        ctx: &Context<'_>,
        input: CreatePlaylistInput,
    ) -> Result<Playlist> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        PlaylistPolicy::check(Some(user), PolicyAction::Create, None).authorize()?;

        let db = ctx.data::<DatabaseConnection>()?;

        let playlist = CreatePlaylistAction::create(
            db,
            CreatePlaylistActionParameters {
                name: input.name,
                description: input.description,
                visibility: input.visibility,
                user_id: user.user.clone().id,
            },
        )
        .await?;

        Ok(playlist.into())
    }

    async fn update_playlist(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdatePlaylistInput,
    ) -> Result<Playlist> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let playlist = playlist::Entity::find()
            .filter(playlist::Column::Hashid.eq(id))
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        PlaylistPolicy::check(Some(user), PolicyAction::Update, Some(&playlist)).authorize()?;

        let playlist = UpdatePlaylistAction::update(
            db,
            playlist,
            UpdatePlaylistActionParameters {
                name: input.name,
                description: input.description,
                visibility: input.visibility,
            },
        )
        .await?;

        Ok(playlist.into())
    }

    async fn delete_playlist(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let playlist = playlist::Entity::find()
            .filter(playlist::Column::Hashid.eq(id))
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        PlaylistPolicy::check(Some(user), PolicyAction::Delete, Some(&playlist)).authorize()?;

        let result = DeletePlaylistAction::delete(db, playlist).await?;

        Ok(result)
    }
}
