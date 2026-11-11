use async_graphql::connection::{Connection, EmptyFields, OpaqueCursor};
use async_graphql::{Context, InputObject, Object, Result, ResultExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter};

use crate::entities::content::studio;
use crate::graphql::cursor::{CursorSort, CursorSortKey, PaginationCursor, cursor_paginate};
use crate::graphql::enums::sort::GraphQLSort;
use crate::graphql::enums::sort::content::studio_sort::StudioSort;
use crate::graphql::inputs::pagination_input::PaginationInput;
use crate::graphql::types::content::studio::Studio;
use crate::scopes::without_trashed;

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
        if let Some(pagination) = &pagination {
            pagination.validate(ctx).extend()?;
        }

        let mut query = studio::Entity::find().filter(without_trashed::<studio::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(name_like) = filter.name_like {
            query = query.filter(studio::Column::Name.like(name_like))
        }

        let mut cursor_sorts = sort
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(StudioSort::cursor_sort)
            .collect::<Vec<_>>();

        if !cursor_sorts
            .iter()
            .any(|sort| matches!(&sort.key, CursorSortKey::Column(studio::Column::Id)))
        {
            cursor_sorts.push(CursorSort {
                key: CursorSortKey::Column(studio::Column::Id),
                order: Order::Asc,
            });
        }

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
