use crate::{entities::content::image, enums::content::imagefacet::ImageFacet};
use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields, OpaqueCursor},
};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter};

use crate::{
    graphql::{
        cursor::{CursorSort, PaginationCursor, cursor_paginate},
        enums::sort::{GraphQLSort, content::image_sort::ImageSort},
        inputs::pagination_input::PaginationInput,
        types::content::image::Image,
    },
    scopes::without_trashed,
};

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
        let mut query = image::Entity::find().filter(without_trashed::<image::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(facet) = filter.facet {
            query = query.filter(image::Column::Facet.eq(facet));
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
            .filter_map(ImageSort::cursor_sort)
            .collect::<Vec<_>>();

        cursor_sorts.push(CursorSort {
            column: image::Column::Id,
            order: Order::Asc,
        });

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
