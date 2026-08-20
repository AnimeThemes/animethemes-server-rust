use async_graphql::{Context, Object, Result};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::admin::announcement, graphql::types::admin::announcement::Announcement,
    scopes::admin::announcement::current_announcement,
};

#[derive(Default)]
pub struct AnnouncementQuery;

#[Object]
impl AnnouncementQuery {
    async fn current_announcements(&self, ctx: &Context<'_>) -> Result<Vec<Announcement>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let query = announcement::Entity::find().filter(current_announcement());

        let result = query.all(db).await?;

        Ok(result.into_iter().map(Into::into).collect())
    }
}
