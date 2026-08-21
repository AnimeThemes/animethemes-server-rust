use async_graphql::{
    Object, OutputType,
    connection::{ConnectionNameType, EdgeNameType},
};
use chrono::{DateTime, Utc};

use crate::graphql::utils::format_datetime;

pub struct AnimeStudioEdgeFields {
    /// The date that the resource was created
    pub created_at: DateTime<Utc>,
    /// The date that the resource was updated
    pub updated_at: DateTime<Utc>,
}

#[Object]
impl AnimeStudioEdgeFields {
    async fn created_at(&self, #[graphql(default = "%+")] format: String) -> String {
        format_datetime(&self.created_at, &format)
    }

    async fn updated_at(&self, #[graphql(default = "%+")] format: String) -> String {
        format_datetime(&self.updated_at, &format)
    }
}

pub struct AnimeStudioEdge;

impl EdgeNameType for AnimeStudioEdge {
    fn type_name<T: OutputType>() -> String {
        "AnimeStudioEdge".to_string()
    }
}

pub struct AnimeStudioConnection;

impl ConnectionNameType for AnimeStudioConnection {
    fn type_name<T: OutputType>() -> String {
        "AnimeStudioConnection".to_string()
    }
}
