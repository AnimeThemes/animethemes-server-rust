use async_graphql::connection::{Connection, EmptyFields, OpaqueCursor};
use async_graphql::{Context, InputObject, Object, Result, ResultExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter};

use crate::entities::content::series;
use crate::graphql::cursor::{CursorSort, CursorSortKey, PaginationCursor, cursor_paginate};
use crate::graphql::enums::sort::GraphQLSort;
use crate::graphql::enums::sort::content::series_sort::SeriesSort;
use crate::graphql::inputs::pagination_input::PaginationInput;
use crate::graphql::types::content::series::Series;
use crate::scopes::without_trashed;

#[derive(InputObject, Default)]
struct SeriesFilterInput {
    title_romaji_like: Option<String>,
}

#[derive(Default)]
pub struct SeriesQuery;

#[Object]
impl SeriesQuery {
    async fn series(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Series>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let series = series::Entity::find()
            .filter(without_trashed::<series::Entity>())
            .filter(series::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(series.map(Into::into))
    }

    async fn series_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<SeriesFilterInput>,
        sort: Option<Vec<SeriesSort>>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Series, EmptyFields, EmptyFields>> {
        if let Some(pagination) = &pagination {
            pagination.validate(ctx).extend()?;
        }

        let mut query = series::Entity::find().filter(without_trashed::<series::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(title_romaji_like) = filter.title_romaji_like {
            query = query.filter(series::Column::Title.like(title_romaji_like))
        }

        let mut cursor_sorts = sort
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(SeriesSort::cursor_sort)
            .collect::<Vec<_>>();

        if !cursor_sorts
            .iter()
            .any(|sort| matches!(&sort.key, CursorSortKey::Column(series::Column::Id)))
        {
            cursor_sorts.push(CursorSort {
                key: CursorSortKey::Column(series::Column::Id),
                order: Order::Asc,
            });
        }

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
