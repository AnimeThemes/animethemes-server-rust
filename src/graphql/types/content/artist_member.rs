use async_graphql::{
    ComplexObject, OutputType, SimpleObject,
    connection::{ConnectionNameType, EdgeNameType},
};
use chrono::{DateTime, Utc};

use crate::graphql::utils::format_datetime;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ArtistMemberEdgeFields {
    /// Used to distinguish member by alias
    pub alias: Option<String>,
    /// Used to distinguish member by character
    pub r#as: Option<String>,
    /// Used to extra annotation, like member role
    pub notes: Option<String>,
    /// Used to determine the relevance order of members in group
    pub relevance: i32,

    #[graphql(skip)]
    pub created_at: Option<DateTime<Utc>>,

    #[graphql(skip)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl ArtistMemberEdgeFields {
    /// The date that the resource was created
    async fn created_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.created_at.as_ref(), &format)
    }

    /// The date that the resource was updated
    async fn updated_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.updated_at.as_ref(), &format)
    }
}

pub struct ArtistMemberEdge;

impl EdgeNameType for ArtistMemberEdge {
    fn type_name<T: OutputType>() -> String {
        "ArtistMemberEdge".to_string()
    }
}

pub struct ArtistMemberConnection;

impl ConnectionNameType for ArtistMemberConnection {
    fn type_name<T: OutputType>() -> String {
        "ArtistMemberConnection".to_string()
    }
}
