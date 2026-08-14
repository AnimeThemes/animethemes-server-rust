use async_graphql::SimpleObject;

use crate::entities::admin::announcement;

/// Represents a site-wide message to be broadcasted on the homepage.
#[derive(SimpleObject)]
pub struct Announcement {
    /// The primary key of the resource
    pub id: u64,
    /// The announcement text
    pub content: String,
}

impl From<announcement::Model> for Announcement {
    fn from(model: announcement::Model) -> Self {
        Self {
            id: model.id,
            content: model.content,
        }
    }
}
