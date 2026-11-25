use async_graphql::connection::{ConnectionNameType, EdgeNameType};
use async_graphql::{Object, OutputType, Result};

use crate::graphql::utils::format_datetime;

pub struct StudioAnimeEdgeFields {
    /// The date that the resource was created
    pub created_at: time::OffsetDateTime,
    /// The date that the resource was updated
    pub updated_at: time::OffsetDateTime,
}

#[Object]
impl StudioAnimeEdgeFields {
    async fn created_at(
        &self,
        #[graphql(default = "%Y-%m-%d %H:%M:%S")] format: String,
    ) -> Result<String> {
        format_datetime(&self.created_at, &format)
    }

    async fn updated_at(
        &self,
        #[graphql(default = "%Y-%m-%d %H:%M:%S")] format: String,
    ) -> Result<String> {
        format_datetime(&self.updated_at, &format)
    }
}

pub struct StudioAnimeEdge;

impl EdgeNameType for StudioAnimeEdge {
    fn type_name<T: OutputType>() -> String {
        "StudioAnimeEdge".to_string()
    }
}

pub struct StudioAnimeConnection;

impl ConnectionNameType for StudioAnimeConnection {
    fn type_name<T: OutputType>() -> String {
        "StudioAnimeConnection".to_string()
    }
}
