use sea_orm::{DatabaseConnection, LoaderTrait};
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::{animethemeentry, video},
    typesense::{
        documents::{
            HasId,
            animethemeentry_document::{AnimeThemeEntryDocument, build_animethemeentry_documents},
        },
        index_document::BuildDocumentsFuture,
    },
};

pub const QUERY_BY: &str = "filename,tags,entries.animetheme.song.title,entries.animetheme.song.title_native,entries.animetheme.anime.title,entries.animetheme.anime.title_english,entries.animetheme.anime.title_native,entries.animetheme.anime.synonyms,entries.type_sequence_version";
pub const QUERY_BY_WEIGHTS: &str = "10,8,6,7,5,5,5,4,4";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "videos", enable_nested_fields = true)]
pub struct VideoDocument {
    pub id: String,
    pub filename: String,
    pub tags: String,
    pub entries: Vec<AnimeThemeEntryDocument>,
    pub created_at: Option<i64>,
}

impl HasId for VideoDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

type VideoDocumentFrom = (video::Model, Vec<AnimeThemeEntryDocument>);

impl From<VideoDocumentFrom> for VideoDocument {
    fn from((model, entry_documents): VideoDocumentFrom) -> Self {
        Self {
            id: model.id.to_string(),
            filename: model.filename.clone(),
            tags: model.tags(),
            entries: entry_documents,
            created_at: model.created_at.map(|c| c.timestamp()),
        }
    }
}

pub fn build_video_documents<'a>(
    models: Vec<video::Model>,
    database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, VideoDocument> {
    Box::pin(async move {
        let entry_models: Vec<Vec<animethemeentry::Model>> =
            models.load_many(animethemeentry::Entity, database).await?;

        let mut entry_documents: Vec<Vec<AnimeThemeEntryDocument>> =
            Vec::with_capacity(entry_models.len());

        for entry_group in entry_models {
            entry_documents.push(build_animethemeentry_documents(entry_group, database).await?);
        }

        let documents = models
            .into_iter()
            .zip(entry_documents)
            .map(|(model, entry_documents)| VideoDocument::from((model, entry_documents)))
            .collect();

        Ok(documents)
    })
}
