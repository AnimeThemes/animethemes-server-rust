use async_graphql::{
    Object, OutputType,
    connection::{ConnectionNameType, EdgeNameType},
};
use chrono::{DateTime, Utc};

use crate::graphql::utils::format_datetime;

pub struct EntryVideoEdgeFields {
    /// The date that the resource was created
    pub created_at: DateTime<Utc>,
    /// The date that the resource was updated
    pub updated_at: DateTime<Utc>,
}

#[Object]
impl EntryVideoEdgeFields {
    async fn created_at(&self, #[graphql(default = "%+")] format: String) -> String {
        format_datetime(&self.created_at, &format)
    }

    async fn updated_at(&self, #[graphql(default = "%+")] format: String) -> String {
        format_datetime(&self.updated_at, &format)
    }
}

pub struct EntryVideoEdge;

impl EdgeNameType for EntryVideoEdge {
    fn type_name<T: OutputType>() -> String {
        "EntryVideoEdge".to_string()
    }
}

pub struct EntryVideoConnection;

impl ConnectionNameType for EntryVideoConnection {
    fn type_name<T: OutputType>() -> String {
        "EntryVideoConnection".to_string()
    }
}

pub struct VideoEntryEdgeFields {
    /// The date that the resource was created
    pub created_at: DateTime<Utc>,
    /// The date that the resource was updated
    pub updated_at: DateTime<Utc>,
}

#[Object]
impl VideoEntryEdgeFields {
    async fn created_at(&self, #[graphql(default = "%+")] format: String) -> String {
        format_datetime(&self.created_at, &format)
    }

    async fn updated_at(&self, #[graphql(default = "%+")] format: String) -> String {
        format_datetime(&self.updated_at, &format)
    }
}

pub struct VideoEntryEdge;

impl EdgeNameType for VideoEntryEdge {
    fn type_name<T: OutputType>() -> String {
        "VideoEntryEdge".to_string()
    }
}

pub struct VideoEntryConnection;

impl ConnectionNameType for VideoEntryConnection {
    fn type_name<T: OutputType>() -> String {
        "VideoEntryConnection".to_string()
    }
}
