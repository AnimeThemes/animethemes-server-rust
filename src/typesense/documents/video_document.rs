use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::entities::content::video;
use crate::typesense::documents::HasId;
use crate::typesense::index_document::BuildDocumentsFuture;

pub const QUERY_BY: &str = "filename,tags";
pub const QUERY_BY_WEIGHTS: &str = "10,4";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "videos")]
pub struct VideoDocument {
    pub id: String,
    pub filename: String,
    pub tags: String,
    pub created_at: i64,
}

impl HasId for VideoDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

type VideoDocumentFrom = video::Model;

impl From<VideoDocumentFrom> for VideoDocument {
    fn from(model: VideoDocumentFrom) -> Self {
        Self {
            id: model.id.to_string(),
            filename: model.filename.clone(),
            tags: model.tags(),
            created_at: model.created_at.unix_timestamp(),
        }
    }
}

pub fn build_video_documents<'a>(
    models: Vec<video::Model>,
    _database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, VideoDocument> {
    Box::pin(async move {
        let documents = models.into_iter().map(VideoDocument::from).collect();

        Ok(documents)
    })
}
