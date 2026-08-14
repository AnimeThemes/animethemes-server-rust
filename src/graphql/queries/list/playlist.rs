use async_graphql::{
    Context, Error, InputObject, Object, Result,
    connection::{Connection, EmptyFields},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::list::playlist,
    graphql::{
        enums::sort::{GraphQLSort, list::playlist_sort::PlaylistSort},
        inputs::pagination_input::PaginationInput,
        types::list::playlist::Playlist,
        utils::cursor_paginate,
    },
    middlewares::current_user::CurrentUser,
    policies::{AppError, Policy, PolicyAction, list::playlist::PlaylistPolicy},
    scopes::list::playlist::public_playlists,
};

#[derive(InputObject, Default)]
struct PlaylistFilterInput {
    name_like: Option<String>,
}

#[derive(Default)]
pub struct PlaylistQuery;

#[Object]
impl PlaylistQuery {
    async fn playlist(&self, ctx: &Context<'_>, id: String) -> Result<Option<Playlist>> {
        let user = ctx.data::<CurrentUser>().ok();

        let db = ctx.data::<DatabaseConnection>()?;

        let playlist = playlist::Entity::find()
            .filter(playlist::Column::Hashid.eq(id))
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        PlaylistPolicy::check(user, PolicyAction::View, Some(&playlist)).authorize()?;

        Ok(Some(playlist.into()))
    }

    async fn playlist_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<PlaylistFilterInput>,
        sort: Option<Vec<PlaylistSort>>,
    ) -> Result<Connection<u64, Playlist, EmptyFields, EmptyFields>> {
        let user = ctx.data::<CurrentUser>().ok();

        PlaylistPolicy::check(user, PolicyAction::ViewAny, None).authorize()?;

        let mut query = playlist::Entity::find().filter(public_playlists());

        let filter = filter.unwrap_or_default();

        if let Some(name_like) = filter.name_like {
            query = query.filter(playlist::Column::Name.like(name_like))
        }

        if let Some(sorts) = sort {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        cursor_paginate(
            query,
            ctx,
            playlist::Column::Id,
            pagination,
            |model: &playlist::Model| model.id,
        )
        .await
    }
}
