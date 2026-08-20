use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields, OpaqueCursor},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter};

use crate::{
    entities::content::series,
    graphql::{
        cursor::{CursorSort, PaginationCursor, cursor_paginate},
        enums::sort::{GraphQLSort, content::series_sort::SeriesSort},
        inputs::pagination_input::PaginationInput,
        types::content::series::Series,
    },
    scopes::without_trashed,
};

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
        let mut query = series::Entity::find().filter(without_trashed::<series::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(title_romaji_like) = filter.title_romaji_like {
            query = query.filter(series::Column::Title.like(title_romaji_like))
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
            .filter_map(SeriesSort::cursor_sort)
            .collect::<Vec<_>>();

        cursor_sorts.push(CursorSort {
            column: series::Column::Id,
            order: Order::Asc,
        });

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
