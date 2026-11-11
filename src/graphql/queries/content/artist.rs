use async_graphql::connection::{Connection, EmptyFields, OpaqueCursor};
use async_graphql::{Context, InputObject, Object, Result, ResultExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter};

use crate::entities::content::artist;
use crate::graphql::cursor::{CursorSort, CursorSortKey, PaginationCursor, cursor_paginate};
use crate::graphql::enums::sort::GraphQLSort;
use crate::graphql::enums::sort::content::artist_sort::ArtistSort;
use crate::graphql::inputs::pagination_input::PaginationInput;
use crate::graphql::types::content::artist::Artist;
use crate::scopes::without_trashed;

#[derive(InputObject, Default)]
struct ArtistFilterInput {
    name_main_like: Option<String>,
}

#[derive(Default)]
pub struct ArtistQuery;

#[Object]
impl ArtistQuery {
    async fn artist(&self, ctx: &Context<'_>, slug: String) -> Result<Option<Artist>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let artist = artist::Entity::find()
            .filter(without_trashed::<artist::Entity>())
            .filter(artist::Column::Slug.eq(slug))
            .one(db)
            .await?;

        Ok(artist.map(Into::into))
    }

    async fn artist_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<ArtistFilterInput>,
        sort: Option<Vec<ArtistSort>>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Artist, EmptyFields, EmptyFields>> {
        if let Some(pagination) = &pagination {
            pagination.validate(ctx).extend()?;
        }

        let mut query = artist::Entity::find().filter(without_trashed::<artist::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(name_main_like) = filter.name_main_like {
            query = query.filter(artist::Column::Name.like(name_main_like))
        }

        let mut cursor_sorts = sort
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(ArtistSort::cursor_sort)
            .collect::<Vec<_>>();

        if !cursor_sorts
            .iter()
            .any(|sort| matches!(&sort.key, CursorSortKey::Column(artist::Column::Id)))
        {
            cursor_sorts.push(CursorSort {
                key: CursorSortKey::Column(artist::Column::Id),
                order: Order::Asc,
            });
        }

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
