use async_graphql::{
    Object, OutputType,
    connection::{ConnectionNameType, EdgeNameType},
};
use chrono::{DateTime, Utc};

use crate::graphql::utils::format_datetime;

pub struct AnimeThemeEntryVideoEdgeFields {
    /// The date that the resource was created
    pub created_at: Option<DateTime<Utc>>,
    /// The date that the resource was updated
    pub updated_at: Option<DateTime<Utc>>,
}

#[Object]
impl AnimeThemeEntryVideoEdgeFields {
    async fn created_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.created_at.as_ref(), &format)
    }

    async fn updated_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.updated_at.as_ref(), &format)
    }
}

pub struct AnimeThemeEntryVideoEdge;

impl EdgeNameType for AnimeThemeEntryVideoEdge {
    fn type_name<T: OutputType>() -> String {
        "AnimeThemeEntryVideoEdge".to_string()
    }
}

pub struct AnimeThemeEntryVideoConnection;

impl ConnectionNameType for AnimeThemeEntryVideoConnection {
    fn type_name<T: OutputType>() -> String {
        "AnimeThemeEntryVideoConnection".to_string()
    }
}

pub struct VideoAnimeThemeEntryEdgeFields {
    /// The date that the resource was created
    pub created_at: Option<DateTime<Utc>>,
    /// The date that the resource was updated
    pub updated_at: Option<DateTime<Utc>>,
}

#[Object]
impl VideoAnimeThemeEntryEdgeFields {
    async fn created_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.created_at.as_ref(), &format)
    }

    async fn updated_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.updated_at.as_ref(), &format)
    }
}

pub struct VideoAnimeThemeEntryEdge;

impl EdgeNameType for VideoAnimeThemeEntryEdge {
    fn type_name<T: OutputType>() -> String {
        "VideoAnimeThemeEntryEdge".to_string()
    }
}

pub struct VideoAnimeThemeEntryConnection;

impl ConnectionNameType for VideoAnimeThemeEntryConnection {
    fn type_name<T: OutputType>() -> String {
        "VideoAnimeThemeEntryConnection".to_string()
    }
}
