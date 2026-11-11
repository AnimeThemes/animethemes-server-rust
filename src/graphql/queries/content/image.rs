use async_graphql::connection::{Connection, EmptyFields, OpaqueCursor};
use async_graphql::{Context, InputObject, Object, Result, ResultExt};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter};

use crate::entities::content::image;
use crate::enums::content::imagefacet::ImageFacet;
use crate::graphql::cursor::{CursorSort, CursorSortKey, PaginationCursor, cursor_paginate};
use crate::graphql::enums::sort::GraphQLSort;
use crate::graphql::enums::sort::content::image_sort::ImageSort;
use crate::graphql::inputs::pagination_input::PaginationInput;
use crate::graphql::types::content::image::Image;
use crate::scopes::without_trashed;

#[derive(InputObject, Default)]
struct ImageFilterInput {
    facet: Option<ImageFacet>,
}

#[derive(Default)]
pub struct ImageQuery;

#[Object]
impl ImageQuery {
    async fn image_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<ImageFilterInput>,
        sort: Option<Vec<ImageSort>>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Image, EmptyFields, EmptyFields>> {
        if let Some(pagination) = &pagination {
            pagination.validate(ctx).extend()?;
        }

        let mut query = image::Entity::find().filter(without_trashed::<image::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(facet) = filter.facet {
            query = query.filter(image::Column::Facet.eq(facet));
        }

        let mut cursor_sorts = sort
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(ImageSort::cursor_sort)
            .collect::<Vec<_>>();

        if !cursor_sorts
            .iter()
            .any(|sort| matches!(&sort.key, CursorSortKey::Column(image::Column::Id)))
        {
            cursor_sorts.push(CursorSort {
                key: CursorSortKey::Column(image::Column::Id),
                order: Order::Asc,
            });
        }

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
