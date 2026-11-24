use sea_orm::{DatabaseConnection, LoaderTrait};
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::entities::content::{anime, series};
use crate::typesense::documents::HasId;
use crate::typesense::documents::anime_document::{AnimeDocument, build_anime_documents};
use crate::typesense::index_document::BuildDocumentsFuture;

pub const QUERY_BY: &str = "title,anime.title_english,anime.title_native,anime.synonyms";
pub const QUERY_BY_WEIGHTS: &str = "10,8,8,6";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "series", enable_nested_fields = true)]
pub struct SeriesDocument {
    pub id: String,
    #[typesense(sort)]
    pub title: String,
    pub anime: Vec<AnimeDocument>,
    #[typesense(sort)]
    pub created_at: i64,
}

impl HasId for SeriesDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

type SeriesDocumentFrom = (series::Model, Vec<AnimeDocument>);

impl From<SeriesDocumentFrom> for SeriesDocument {
    fn from((model, anime_documents): SeriesDocumentFrom) -> Self {
        Self {
            id: model.id.to_string(),
            title: model.title,
            anime: anime_documents,
            created_at: model.created_at.timestamp(),
        }
    }
}

pub fn build_series_documents<'a>(
    models: Vec<series::Model>,
    database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, SeriesDocument> {
    Box::pin(async move {
        let anime_groups: Vec<Vec<anime::Model>> =
            models.load_many(anime::Entity, database).await?;

        let counts: Vec<usize> = anime_groups.iter().map(Vec::len).collect();

        let mut anime_documents =
            build_anime_documents(anime_groups.into_iter().flatten().collect(), database)
                .await?
                .into_iter();

        let documents = models
            .into_iter()
            .zip(counts)
            .map(|(model, count)| {
                let animes = anime_documents.by_ref().take(count).collect();

                SeriesDocument::from((model, animes))
            })
            .collect();

        Ok(documents)
    })
}
