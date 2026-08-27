use async_graphql::{
    Context, Error, InputObject, Object, Result,
    connection::{Connection, EmptyFields, OpaqueCursor},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter};

use crate::{
    AppError,
    entities::list::playlist,
    graphql::{
        cursor::{CursorSort, PaginationCursor, cursor_paginate},
        enums::sort::{GraphQLSort, list::playlist_sort::PlaylistSort},
        inputs::pagination_input::PaginationInput,
        types::list::playlist::Playlist,
    },
    middlewares::current_user::CurrentUser,
    policies::{Policy, PolicyAction, list::playlist::PlaylistPolicy},
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
        let user = ctx.data_opt::<CurrentUser>();

        let db = ctx.data::<DatabaseConnection>()?;

        let playlist = playlist::Entity::find_by_hashid(id)
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
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Playlist, EmptyFields, EmptyFields>>
    {
        let user = ctx.data_opt::<CurrentUser>();

        PlaylistPolicy::check(user, PolicyAction::ViewAny, None).authorize()?;

        let mut query = playlist::Entity::find().filter(public_playlists());

        let filter = filter.unwrap_or_default();

        if let Some(name_like) = filter.name_like {
            query = query.filter(playlist::Column::Name.like(name_like))
        }

        if let Some(sorts) = sort.clone() {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        let mut cursor_sorts = sort
            .clone()
            .unwrap_or(vec![])
            .iter()
            .filter_map(PlaylistSort::cursor_sort)
            .collect::<Vec<_>>();

        cursor_sorts.push(CursorSort {
            column: playlist::Column::Id,
            order: Order::Asc,
        });

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
