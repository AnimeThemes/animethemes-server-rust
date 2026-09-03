use crate::{entities::content::entry, scopes::without_trashed};
use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields, OpaqueCursor},
};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};

use crate::graphql::{
    cursor::{CursorSort, PaginationCursor, cursor_paginate},
    inputs::pagination_input::PaginationInput,
    types::content::entry::Entry,
};

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
        let mut query = entry::Entity::find().filter(without_trashed::<entry::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(spoiler) = filter.spoiler {
            query = query.filter(entry::Column::Spoiler.eq(spoiler));
        }

        query = query.order_by_desc(entry::Column::TracksCount);

        let cursor_sorts = vec![CursorSort {
            column: entry::Column::CreatedAt,
            order: Order::Asc,
        }];

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
