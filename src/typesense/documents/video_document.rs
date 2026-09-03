use sea_orm::{DatabaseConnection, LoaderTrait};
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::{entry, video},
    typesense::{
        documents::{
            HasId,
            entry_document::{EntryDocument, build_entry_documents},
        },
        index_document::BuildDocumentsFuture,
    },
};

pub const QUERY_BY: &str = "filename,tags,entries.theme.song.title,entries.theme.song.title_native,entries.theme.anime.title,entries.theme.anime.title_english,entries.theme.anime.title_native,entries.theme.anime.synonyms,entries.type_sequence_version";
pub const QUERY_BY_WEIGHTS: &str = "10,8,6,7,5,5,5,4,4";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "videos", enable_nested_fields = true)]
pub struct VideoDocument {
    pub id: String,
    pub filename: String,
    pub tags: String,
    pub entries: Vec<EntryDocument>,
    pub created_at: i64,
}

impl HasId for VideoDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

type VideoDocumentFrom = (video::Model, Vec<EntryDocument>);

impl From<VideoDocumentFrom> for VideoDocument {
    fn from((model, entry_documents): VideoDocumentFrom) -> Self {
        Self {
            id: model.id.to_string(),
            filename: model.filename.clone(),
            tags: model.tags(),
            entries: entry_documents,
            created_at: model.created_at.timestamp(),
        }
    }
}

pub fn build_video_documents<'a>(
    models: Vec<video::Model>,
    database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, VideoDocument> {
    Box::pin(async move {
        let entry_models: Vec<Vec<entry::Model>> =
            models.load_many(entry::Entity, database).await?;

        let mut entry_documents: Vec<Vec<EntryDocument>> = Vec::with_capacity(entry_models.len());

        for entry_group in entry_models {
            entry_documents.push(build_entry_documents(entry_group, database).await?);
        }

        let documents = models
            .into_iter()
            .zip(entry_documents)
            .map(|(model, entry_documents)| VideoDocument::from((model, entry_documents)))
            .collect();

        Ok(documents)
    })
}
