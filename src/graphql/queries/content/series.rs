use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::content::series,
    graphql::{
        enums::sort::{GraphQLSort, content::series_sort::SeriesSort},
        inputs::pagination_input::PaginationInput,
        types::content::series::Series,
        utils::cursor_paginate,
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

        Ok(series.map(|a| a.into()))
    }

    async fn series_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<SeriesFilterInput>,
        sort: Option<Vec<SeriesSort>>,
    ) -> Result<Connection<u64, Series, EmptyFields, EmptyFields>> {
        let mut query = series::Entity::find().filter(without_trashed::<series::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(title_romaji_like) = filter.title_romaji_like {
            query = query.filter(series::Column::Title.like(title_romaji_like))
        }

        if let Some(sorts) = sort {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        cursor_paginate(
            query,
            ctx,
            series::Column::Id,
            pagination,
            |model: &series::Model| model.id,
        )
        .await
    }
}
