use crate::entities::list::playlist;
use crate::enums::list::playlistvisibility::PlaylistVisibility;
use async_graphql::{Context, Error, InputObject, Object, Result};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
};

use crate::graphql::types::list::playlist::Playlist;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::list::playlist::PlaylistPolicy;
use crate::policies::{AppError, Policy, PolicyAction};

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

        let playlist = playlist::ActiveModel {
            name: Set(input.name),
            description: Set(input.description),
            visibility: Set(input.visibility),
            user_id: Set(user.user.clone().id),
            ..Default::default()
        };

        let playlist = playlist.insert(db).await?;

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

        let mut playlist = playlist::ActiveModel {
            id: Set(playlist.id),
            ..Default::default()
        };

        if let Some(name) = input.name {
            playlist.name = Set(name);
        }

        if let Some(description) = input.description {
            playlist.description = Set(Some(description));
        }

        if let Some(visibility) = input.visibility {
            playlist.visibility = Set(visibility);
        }

        let playlist = playlist.update(db).await?;

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

        let result = playlist.delete(db).await?;

        Ok(result.rows_affected > 0)
    }
}
