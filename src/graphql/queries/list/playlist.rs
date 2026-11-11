use async_graphql::connection::{Connection, EmptyFields, OpaqueCursor};
use async_graphql::{Context, Error, InputObject, Object, Result, ResultExt};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, QueryFilter};

use crate::AppError;
use crate::entities::list::{playlist, track};
use crate::graphql::cursor::{CursorSort, CursorSortKey, PaginationCursor, cursor_paginate};
use crate::graphql::enums::sort::GraphQLSort;
use crate::graphql::enums::sort::list::playlist_sort::PlaylistSort;
use crate::graphql::inputs::pagination_input::PaginationInput;
use crate::graphql::types::list::playlist::Playlist;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::list::playlist::PlaylistPolicy;
use crate::policies::{Policy, PolicyAction};
use crate::scopes::list::playlist::public_playlists;

#[derive(InputObject, Default)]
struct PlaylistFilterInput {
    name_like: Option<String>,
    tracks_exist: Option<bool>,
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
        if let Some(pagination) = &pagination {
            pagination.validate(ctx).extend()?;
        }

        let user = ctx.data_opt::<CurrentUser>();

        PlaylistPolicy::check(user, PolicyAction::ViewAny, None).authorize()?;

        let mut query = playlist::Entity::find().filter(public_playlists());

        let filter = filter.unwrap_or_default();

        if let Some(name_like) = filter.name_like {
            query = query.filter(playlist::Column::Name.like(name_like))
        }

        if filter.tracks_exist == Some(true) {
            query = query.has_related(track::Entity, Condition::all())
        }

        let mut cursor_sorts = sort
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(PlaylistSort::cursor_sort)
            .collect::<Vec<_>>();

        if !cursor_sorts
            .iter()
            .any(|sort| matches!(&sort.key, CursorSortKey::Column(playlist::Column::Id)))
        {
            cursor_sorts.push(CursorSort {
                key: CursorSortKey::Column(playlist::Column::Id),
                order: Order::Asc,
            });
        }

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
