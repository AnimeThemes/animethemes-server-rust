use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::song,
    typesense::{documents::HasId, index_document::BuildDocumentsFuture},
};

pub const QUERY_BY: &str = "title,title_native";
pub const QUERY_BY_WEIGHTS: &str = "10,8";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "songs")]
pub struct SongDocument {
    pub id: String,
    #[typesense(sort)]
    pub title: Option<String>,
    pub title_native: Option<String>,
    pub created_at: i64,
}

impl HasId for SongDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

impl From<song::Model> for SongDocument {
    fn from(model: song::Model) -> Self {
        let title_native = if model.title_native.as_ref() == model.title.as_ref() {
            None
        } else {
            model.title_native.clone()
        };

        Self {
            id: model.id.to_string(),
            title: model.title.clone(),
            title_native: title_native,
            created_at: model.created_at.timestamp(),
        }
    }
}

pub fn build_song_documents<'a>(
    models: Vec<song::Model>,
    _database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, SongDocument> {
    Box::pin(async move {
        let documents = models.into_iter().map(SongDocument::from).collect();

        Ok(documents)
    })
}
