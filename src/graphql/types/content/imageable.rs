use async_graphql::connection::{ConnectionNameType, EdgeNameType};
use async_graphql::{ComplexObject, OutputType, Result, SimpleObject};

use crate::graphql::utils::format_datetime;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ImageableEdgeFields {
    /// Used to sort the images
    pub depth: i32,

    #[graphql(skip)]
    pub created_at: time::OffsetDateTime,
    #[graphql(skip)]
    pub updated_at: time::OffsetDateTime,
}

#[ComplexObject]
impl ImageableEdgeFields {
    /// The date that the resource was created
    async fn created_at(
        &self,
        #[graphql(default = "%Y-%m-%d %H:%M:%S")] format: String,
    ) -> Result<String> {
        format_datetime(&self.created_at, &format)
    }

    /// The date that the resource was updated
    async fn updated_at(
        &self,
        #[graphql(default = "%Y-%m-%d %H:%M:%S")] format: String,
    ) -> Result<String> {
        format_datetime(&self.updated_at, &format)
    }
}

pub struct ImageableEdge;

impl EdgeNameType for ImageableEdge {
    fn type_name<T: OutputType>() -> String {
        "ImageableEdge".to_string()
    }
}

pub struct ImageableConnection;

impl ConnectionNameType for ImageableConnection {
    fn type_name<T: OutputType>() -> String {
        "ImageableConnection".to_string()
    }
}
