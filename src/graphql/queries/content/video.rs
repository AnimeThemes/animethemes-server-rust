use crate::{entities::content::video, graphql::cursor::CursorSortKey};
use async_graphql::{
    Context, InputObject, Object, Result, ResultExt,
    connection::{Connection, EmptyFields, OpaqueCursor},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter};

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
    async fn video(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Video>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let model = video::Entity::find()
            .filter(without_trashed::<video::Entity>())
            .filter(video::Column::Id.eq(id))
            .one(db)
            .await?;

        Ok(model.map(Into::into))
    }

    async fn video_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        filter: Option<VideoFilterInput>,
        sort: Option<Vec<VideoSort>>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Video, EmptyFields, EmptyFields>> {
        if let Some(pagination) = &pagination {
            pagination.validate(ctx).extend()?;
        }

        let mut query = video::Entity::find().filter(without_trashed::<video::Entity>());

        let filter = filter.unwrap_or_default();

        if let Some(nc) = filter.nc {
            query = query.filter(video::Column::Nc.eq(nc))
        }

        let mut cursor_sorts = sort
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(VideoSort::cursor_sort)
            .collect::<Vec<_>>();

        if !cursor_sorts
            .iter()
            .any(|sort| matches!(&sort.key, CursorSortKey::Column(video::Column::Id)))
        {
            cursor_sorts.push(CursorSort {
                key: CursorSortKey::Column(video::Column::Id),
                order: Order::Asc,
            });
        }

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }
}
