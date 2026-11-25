use async_graphql::connection::{ConnectionNameType, EdgeNameType};
use async_graphql::{Object, OutputType, Result};

use crate::graphql::utils::format_datetime;

pub struct AnimeSeriesEdgeFields {
    /// The date that the resource was created
    pub created_at: time::OffsetDateTime,
    /// The date that the resource was updated
    pub updated_at: time::OffsetDateTime,
}

#[Object]
impl AnimeSeriesEdgeFields {
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

pub struct AnimeSeriesEdge;

impl EdgeNameType for AnimeSeriesEdge {
    fn type_name<T: OutputType>() -> String {
        "AnimeSeriesEdge".to_string()
    }
}

pub struct AnimeSeriesConnection;

impl ConnectionNameType for AnimeSeriesConnection {
    fn type_name<T: OutputType>() -> String {
        "AnimeSeriesConnection".to_string()
    }
}

pub struct SeriesAnimeEdgeFields {
    /// The date that the resource was created
    pub created_at: time::OffsetDateTime,
    /// The date that the resource was updated
    pub updated_at: time::OffsetDateTime,
}

#[Object]
impl SeriesAnimeEdgeFields {
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

pub struct SeriesAnimeEdge;

impl EdgeNameType for SeriesAnimeEdge {
    fn type_name<T: OutputType>() -> String {
        "SeriesAnimeEdge".to_string()
    }
}

pub struct SeriesAnimeConnection;

impl ConnectionNameType for SeriesAnimeConnection {
    fn type_name<T: OutputType>() -> String {
        "SeriesAnimeConnection".to_string()
    }
}
