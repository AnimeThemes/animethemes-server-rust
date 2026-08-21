use crate::{entities::content::animethemeentry, scopes::without_trashed};
use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields, OpaqueCursor},
};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};

use crate::graphql::{
    cursor::{CursorSort, PaginationCursor, cursor_paginate},
    inputs::pagination_input::PaginationInput,
    types::content::animethemeentry::AnimeThemeEntry,
};

#[derive(InputObject, Default)]
pub struct AnimeThemeEntryFilterInput {
    spoiler: Option<bool>,
}

#[derive(Default)]
pub struct AnimeThemeEntryQuery;

#[Object]
impl AnimeThemeEntryQuery {
    pub async fn most_popular_entries(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<AnimeThemeEntryFilterInput>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, AnimeThemeEntry, EmptyFields, EmptyFields>>
    {
        let mut query =
            animethemeentry::Entity::find().filter(without_trashed::<animethemeentry::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(spoiler) = filter.spoiler {
            query = query.filter(animethemeentry::Column::Spoiler.eq(spoiler));
        }

        query = query.order_by_desc(animethemeentry::Column::TracksCount);

        let cursor_sorts = vec![CursorSort {
            column: animethemeentry::Column::CreatedAt,
            order: Order::Asc,
        }];

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
