use async_graphql::{
    Context, Object, Result,
    connection::{Connection, EmptyFields},
};
use sea_orm::{EntityTrait, QueryFilter};

use crate::{
    entities::admin::announcement,
    graphql::{
        inputs::pagination_input::PaginationInput, types::admin::announcement::Announcement,
        utils::cursor_paginate,
    },
    scopes::admin::announcement::current_announcement,
};

#[derive(Default)]
pub struct AnnouncementQuery;

#[Object]
impl AnnouncementQuery {
    async fn announcement_connection(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
    ) -> Result<Connection<u64, Announcement, EmptyFields, EmptyFields>> {
        let query = announcement::Entity::find().filter(current_announcement());

        cursor_paginate(
            query,
            ctx,
            announcement::Column::Id,
            pagination,
            |model: &announcement::Model| model.id,
        )
        .await
    }
}
