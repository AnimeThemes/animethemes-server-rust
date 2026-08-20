use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields, OpaqueCursor},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter};

use crate::{
    entities::content::studio,
    graphql::{
        cursor::{CursorSort, PaginationCursor, cursor_paginate},
        enums::sort::{GraphQLSort, content::studio_sort::StudioSort},
        inputs::pagination_input::PaginationInput,
        types::content::studio::Studio,
    },
    scopes::without_trashed,
};

#[derive(InputObject, Default)]
struct StudioFilterInput {
    name_like: Option<String>,
}

#[derive(Default)]
pub struct StudioQuery;

#[Object]
impl StudioQuery {
    async fn studio(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Studio>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let studio: Option<studio::Model> = studio::Entity::find()
            .filter(without_trashed::<studio::Entity>())
            .filter(studio::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(studio.map(Into::into))
    }

    async fn studio_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<StudioFilterInput>,
        sort: Option<Vec<StudioSort>>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Studio, EmptyFields, EmptyFields>> {
        let mut query = studio::Entity::find().filter(without_trashed::<studio::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(name_like) = filter.name_like {
            query = query.filter(studio::Column::Name.like(name_like))
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
            .filter_map(StudioSort::cursor_sort)
            .collect::<Vec<_>>();

        cursor_sorts.push(CursorSort {
            column: studio::Column::Id,
            order: Order::Asc,
        });

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
