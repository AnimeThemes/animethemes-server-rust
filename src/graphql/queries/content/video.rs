use crate::entities::content::video;
use async_graphql::{
    Context, InputObject, Object, Result,
    connection::{Connection, EmptyFields, OpaqueCursor},
};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter};

use crate::{
    graphql::{
        cursor::{CursorSort, PaginationCursor, cursor_paginate},
        enums::sort::{GraphQLSort, content::video_sort::VideoSort},
        inputs::pagination_input::PaginationInput,
        types::content::video::Video,
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
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Video, EmptyFields, EmptyFields>> {
        let mut query = video::Entity::find().filter(without_trashed::<video::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(nc) = filter.nc {
            query = query.filter(video::Column::Nc.eq(nc))
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
            .filter_map(VideoSort::cursor_sort)
            .collect::<Vec<_>>();

        cursor_sorts.push(CursorSort {
            column: video::Column::Id,
            order: Order::Asc,
        });

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
