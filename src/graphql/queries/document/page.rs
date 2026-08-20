use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields, OpaqueCursor},
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, QueryFilter};

use crate::{
    entities::document::page,
    graphql::{
        cursor::{CursorSort, PaginationCursor, cursor_paginate},
        enums::sort::{GraphQLSort, document::page_sort::PageSort},
        inputs::pagination_input::PaginationInput,
        types::document::page::Page,
    },
    scopes::without_trashed,
};

#[derive(InputObject, Default)]
struct PageFilterInput {
    name_like: Option<String>,
}

#[derive(Default)]
pub struct PageQuery;

#[Object]
impl PageQuery {
    async fn page(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Page>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let page = page::Entity::find()
            .filter(without_trashed::<page::Entity>())
            .filter(page::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(page.map(Into::into))
    }

    async fn page_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<PageFilterInput>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Page, EmptyFields, EmptyFields>> {
        let mut query = page::Entity::find().filter(without_trashed::<page::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(name_like) = filter.name_like {
            query = query.filter(page::Column::Name.like(name_like))
        }

        let cursor_sorts = vec![CursorSort {
            column: page::Column::CreatedAt,
            order: Order::Asc,
        }];

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }

    async fn blog_pages(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        sort: Option<Vec<PageSort>>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Page, EmptyFields, EmptyFields>> {
        let mut query = page::Entity::find().filter(without_trashed::<page::Entity>());

        query = query.filter(
            Condition::any()
                .add(page::Column::Slug.like("blog/%"))
                .add(page::Column::Slug.like("status/%")),
        );

        if let Some(sorts) = sort.clone() {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        let mut cursor_sorts = sort
            .clone()
            .unwrap_or(vec![])
            .iter()
            .filter_map(PageSort::cursor_sort)
            .collect::<Vec<_>>();

        cursor_sorts.push(CursorSort {
            column: page::Column::Id,
            order: Order::Asc,
        });

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
