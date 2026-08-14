use animethemes_graphql_rust::{
    entities::content::image, enums::content::imagefacet::ImageFacet as ImageFacetEnum,
};
use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    graphql::{
        enums::{
            content::imagefacet::ImageFacet,
            sort::{GraphQLSort, content::image_sort::ImageSort},
        },
        inputs::pagination_input::PaginationInput,
        types::content::image::Image,
        utils::cursor_paginate,
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
    ) -> Result<Connection<u64, Image, EmptyFields, EmptyFields>> {
        let mut query = image::Entity::find().filter(without_trashed::<image::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(facet) = filter.facet {
            query = query.filter(image::Column::Facet.eq(ImageFacetEnum::from(facet)));
        }

        if let Some(sorts) = sort {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        cursor_paginate(
            query,
            ctx,
            image::Column::Id,
            pagination,
            |model: &image::Model| model.id,
        )
        .await
    }
}
