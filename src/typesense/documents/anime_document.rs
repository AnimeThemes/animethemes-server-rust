use sea_orm::{ActiveEnum, ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::{anime, synonym},
    typesense::{documents::HasId, index_document::BuildDocumentsFuture},
};

pub const QUERY_BY: &str = "title,title_english,title_native,synonyms";
pub const QUERY_BY_WEIGHTS: &str = "8,6,6,5";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "anime")]
pub struct AnimeDocument {
    pub id: String,
    #[typesense(sort)]
    pub title: String,
    pub title_english: Option<String>,
    pub title_native: Option<String>,
    pub format: Option<i32>,
    #[typesense(sort)]
    pub season: Option<i32>,
    #[typesense(sort)]
    pub year: Option<i32>,
    #[typesense(sort)]
    pub created_at: i64,
    pub synonyms: Vec<String>,
}

impl HasId for AnimeDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

type AnimeDocumentFrom = (anime::Model, Vec<synonym::Model>);

impl From<AnimeDocumentFrom> for AnimeDocument {
    fn from((model, synonyms): AnimeDocumentFrom) -> Self {
        let anime::Model {
            title,
            mut title_english,
            mut title_native,
            ..
        } = model;

        if title_english.as_ref() == Some(&title) || title_english == title_native {
            title_english = None;
        }

        if title_native.as_ref() == Some(&title) {
            title_native = None;
        }

        Self {
            id: model.id.to_string(),
            title: title,
            title_english: title_english,
            title_native: title_native,
            format: model.format.map(|f| f.to_value()),
            season: model.season.map(|s| s.to_value()),
            year: model.year,
            created_at: model.created_at.timestamp(),
            synonyms: synonyms.iter().map(|s| s.text.clone()).collect(),
        }
    }
}

pub fn build_anime_documents<'a>(
    models: Vec<anime::Model>,
    database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, AnimeDocument> {
    Box::pin(async move {
        let synonyms: Vec<Vec<synonym::Model>> = models
            .load_many(
                synonym::Entity::find().filter(synonym::Column::SynonymableType.eq("anime")),
                database,
            )
            .await?;

        let documents = models
            .into_iter()
            .zip(synonyms)
            .map(|(model, synonyms)| AnimeDocument::from((model, synonyms)))
            .collect();

        Ok(documents)
    })
}
