use async_graphql::{
    ComplexObject, OutputType, SimpleObject,
    connection::{ConnectionNameType, EdgeNameType},
};
use chrono::{DateTime, Utc};

use crate::graphql::utils::format_datetime;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ResourceableEdgeFields {
    /// Used to distinguish resources that map to the same resourceable
    pub r#as: Option<String>,

    #[graphql(skip)]
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl ResourceableEdgeFields {
    /// The date that the resource was created
    async fn created_at(&self, #[graphql(default = "%+")] format: String) -> String {
        format_datetime(&self.created_at, &format)
    }

    async fn updated_at(&self, #[graphql(default = "%+")] format: String) -> String {
        format_datetime(&self.updated_at, &format)
    }
}

pub struct ResourceableEdge;

impl EdgeNameType for ResourceableEdge {
    fn type_name<T: OutputType>() -> String {
        "ResourceableEdge".to_string()
    }
}

pub struct ResourceableConnection;

impl ConnectionNameType for ResourceableConnection {
    fn type_name<T: OutputType>() -> String {
        "ResourceableConnection".to_string()
    }
}
