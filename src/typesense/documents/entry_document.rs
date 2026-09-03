use sea_orm::{DatabaseConnection, LoaderTrait};
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::{entry, theme},
    typesense::{
        documents::{
            HasId,
            theme_document::{ThemeDocument, build_theme_documents},
        },
        index_document::BuildDocumentsFuture,
    },
};

pub const QUERY_BY: &str = "theme.song.title,theme.song.title_native,theme.anime.title,type_sequence_version,theme.anime.title_english,theme.anime.title_native,theme.anime.synonyms";
pub const QUERY_BY_WEIGHTS: &str = "10,8,6,6,5,5,4";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "entries", enable_nested_fields = true)]
pub struct EntryDocument {
    pub id: String,
    pub version: String,
    pub type_sequence_version: String,
    pub theme: ThemeDocument,
    pub created_at: i64,
}

impl HasId for EntryDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

type EntryDocumentFrom = (entry::Model, ThemeDocument);

impl From<EntryDocumentFrom> for EntryDocument {
    fn from((model, theme_document): EntryDocumentFrom) -> Self {
        let version = format!("v{}", model.version);
        Self {
            id: model.id.to_string(),
            version: version.clone(),
            type_sequence_version: format!("{}{}", theme_document.type_sequence, version),
            theme: theme_document,
            created_at: model.created_at.timestamp(),
        }
    }
}

pub fn build_entry_documents<'a>(
    models: Vec<entry::Model>,
    database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, EntryDocument> {
    Box::pin(async move {
        let theme_models: Vec<theme::Model> = models
            .load_one(theme::Entity, database)
            .await?
            .into_iter()
            .map(|theme| theme.expect("Theme not found for entry"))
            .collect();

        let theme_documents = build_theme_documents(theme_models, database).await?;

        let documents = models
            .into_iter()
            .zip(theme_documents)
            .map(|(model, theme_document)| EntryDocument::from((model, theme_document)))
            .collect();

        Ok(documents)
    })
}
