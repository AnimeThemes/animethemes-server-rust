use animethemes_server_rust::entities::content::video;
use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    graphql::{
        enums::sort::{GraphQLSort, content::video_sort::VideoSort},
        inputs::pagination_input::PaginationInput,
        types::content::video::Video,
        utils::cursor_paginate,
    },
    scopes::without_trashed,
};

#[derive(InputObject, Default)]
struct VideoFilterInput {
    nc: Option<bool>,
}

#[derive(Default)]
pub struct VideoQuery;

#[Object]
impl VideoQuery {
    async fn video_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<VideoFilterInput>,
        sort: Option<Vec<VideoSort>>,
    ) -> Result<Connection<u64, Video, EmptyFields, EmptyFields>> {
        let mut query = video::Entity::find().filter(without_trashed::<video::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(nc) = filter.nc {
            query = query.filter(video::Column::Nc.eq(nc))
        }

        if let Some(sorts) = sort {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        cursor_paginate(
            query,
            ctx,
            video::Column::Id,
            pagination,
            |model: &video::Model| model.id,
        )
        .await
    }
}
