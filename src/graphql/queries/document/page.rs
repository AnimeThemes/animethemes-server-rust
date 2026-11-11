use async_graphql::connection::{Connection, EmptyFields, OpaqueCursor};
use async_graphql::{Context, InputObject, Object, Result, ResultExt};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, QueryFilter};

use crate::entities::auth::role;
use crate::entities::document::{page, page_roles};
use crate::graphql::cursor::{CursorSort, CursorSortKey, PaginationCursor, cursor_paginate};
use crate::graphql::enums::sort::GraphQLSort;
use crate::graphql::enums::sort::document::page_sort::PageSort;
use crate::graphql::inputs::pagination_input::PaginationInput;
use crate::graphql::types::document::page::Page;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::document::page::PagePolicy;
use crate::policies::{Policy, PolicyAction};
use crate::scopes::document::page::public_pages;
use crate::scopes::without_trashed;

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

        let user = ctx.data_opt::<CurrentUser>();

        let Some(page) = page::Entity::find()
            .filter(without_trashed::<page::Entity>())
            .filter(page::Column::Slug.eq(slug))
            .one(db)
            .await?
        else {
            return Ok(None);
        };

        let page_roles = page_roles::Entity::find()
            .filter(page_roles::Column::PageId.eq(page.id))
            .find_both_related(role::Entity)
            .all(db)
            .await?;

        if !PagePolicy::check(user, PolicyAction::View, Some((&page, &page_roles))).is_allowed() {
            return Ok(None);
        }

        Ok(Some(page.into()))
    }

    async fn page_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<PageFilterInput>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Page, EmptyFields, EmptyFields>> {
        if let Some(pagination) = &pagination {
            pagination.validate(ctx).extend()?;
        }

        let user = ctx.data_opt::<CurrentUser>();
        let user_roles = user.as_ref().map(|u| u.roles.clone()).unwrap_or_default();

        let mut query = page::Entity::find()
            .filter(without_trashed::<page::Entity>())
            .filter(public_pages(&user_roles));

        let filter = filter.unwrap_or_default();

        if let Some(name_like) = filter.name_like {
            query = query.filter(page::Column::Name.like(name_like))
        }

        let cursor_sorts = vec![CursorSort {
            key: CursorSortKey::Column(page::Column::CreatedAt),
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
        let user = ctx.data_opt::<CurrentUser>();
        let user_roles = user.as_ref().map(|u| u.roles.clone()).unwrap_or_default();

        let mut query = page::Entity::find()
            .filter(without_trashed::<page::Entity>())
            .filter(public_pages(&user_roles));

        query = query.filter(
            Condition::any()
                .add(page::Column::Slug.like("blog/%"))
                .add(page::Column::Slug.like("status/%")),
        );

        let mut cursor_sorts = sort
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(PageSort::cursor_sort)
            .collect::<Vec<_>>();

        if !cursor_sorts
            .iter()
            .any(|sort| matches!(&sort.key, CursorSortKey::Column(page::Column::Id)))
        {
            cursor_sorts.push(CursorSort {
                key: CursorSortKey::Column(page::Column::Id),
                order: Order::Asc,
            });
        }

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
