use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::list::playlist,
    typesense::{documents::HasId, index_document::BuildDocumentsFuture},
};

pub const QUERY_BY: &str = "name";
pub const QUERY_BY_WEIGHTS: &str = "10";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "playlists")]
pub struct PlaylistDocument {
    pub id: String,
    #[typesense(sort)]
    pub name: String,
    pub created_at: i64,
}

impl HasId for PlaylistDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

impl From<playlist::Model> for PlaylistDocument {
    fn from(model: playlist::Model) -> Self {
        Self {
            id: model.id.to_string(),
            name: model.name.clone(),
            created_at: model.created_at.timestamp(),
        }
    }
}

pub fn build_playlist_documents<'a>(
    models: Vec<playlist::Model>,
    _database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, PlaylistDocument> {
    Box::pin(async move {
        let documents = models.into_iter().map(PlaylistDocument::from).collect();

        Ok(documents)
    })
}
