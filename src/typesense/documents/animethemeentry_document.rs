use sea_orm::{DatabaseConnection, LoaderTrait};
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::{animetheme, animethemeentry},
    typesense::{
        documents::{
            HasId,
            animetheme_document::{AnimeThemeDocument, build_animetheme_documents},
        },
        index_document::BuildDocumentsFuture,
    },
};

pub const QUERY_BY: &str = "animetheme.song.title,animetheme.song.title_native,animetheme.anime.title,type_sequence_version,animetheme.anime.title_english,animetheme.anime.title_native,animetheme.anime.synonyms";
pub const QUERY_BY_WEIGHTS: &str = "10,8,6,6,5,5,4";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "animethemeentries", enable_nested_fields = true)]
pub struct AnimeThemeEntryDocument {
    pub id: String,
    pub version: String,
    pub type_sequence_version: String,
    pub animetheme: AnimeThemeDocument,
    pub created_at: Option<i64>,
}

impl HasId for AnimeThemeEntryDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

type AnimeThemeEntryDocumentFrom = (animethemeentry::Model, AnimeThemeDocument);

impl From<AnimeThemeEntryDocumentFrom> for AnimeThemeEntryDocument {
    fn from((model, animetheme_document): AnimeThemeEntryDocumentFrom) -> Self {
        let version = format!("v{}", model.version);
        Self {
            id: model.id.to_string(),
            version: version.clone(),
            type_sequence_version: format!("{}{}", animetheme_document.type_sequence, version),
            animetheme: animetheme_document,
            created_at: model.created_at.map(|c| c.timestamp()),
        }
    }
}

pub fn build_animethemeentry_documents<'a>(
    models: Vec<animethemeentry::Model>,
    database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, AnimeThemeEntryDocument> {
    Box::pin(async move {
        let theme_models: Vec<animetheme::Model> = models
            .load_one(animetheme::Entity, database)
            .await?
            .into_iter()
            .map(|animetheme| animetheme.expect("AnimeTheme not found for animethemeentry"))
            .collect();

        let theme_documents = build_animetheme_documents(theme_models, database).await?;

        let documents = models
            .into_iter()
            .zip(theme_documents)
            .map(|(model, theme_document)| AnimeThemeEntryDocument::from((model, theme_document)))
            .collect();

        Ok(documents)
    })
}
