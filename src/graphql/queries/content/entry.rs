use async_graphql::connection::{Connection, EmptyFields, OpaqueCursor};
use async_graphql::{Context, InputObject, Object, Result, ResultExt};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};

use crate::entities::content::entry;
use crate::graphql::cursor::{CursorSort, CursorSortKey, PaginationCursor, cursor_paginate};
use crate::graphql::inputs::pagination_input::PaginationInput;
use crate::graphql::types::content::entry::Entry;
use crate::scopes::without_trashed;

#[derive(InputObject, Default)]
pub struct EntryFilterInput {
    spoiler: Option<bool>,
}

#[derive(Default)]
pub struct EntryQuery;

#[Object]
impl EntryQuery {
    pub async fn most_popular_entries(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<EntryFilterInput>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Entry, EmptyFields, EmptyFields>> {
        if let Some(pagination) = &pagination {
            pagination.validate(ctx).extend()?;
        }

        let mut query = entry::Entity::find().filter(without_trashed::<entry::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(spoiler) = filter.spoiler {
            query = query.filter(entry::Column::Spoiler.eq(spoiler));
        }

        query = query.order_by_desc(entry::Column::TracksCount);

        let cursor_sorts = vec![
            CursorSort {
                key: CursorSortKey::Column(entry::Column::TracksCount),
                order: Order::Desc,
            },
            CursorSort {
                key: CursorSortKey::Column(entry::Column::CreatedAt),
                order: Order::Asc,
            },
        ];

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
