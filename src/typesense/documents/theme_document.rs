use sea_orm::{ActiveEnum, DatabaseConnection, LoaderTrait};
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::{anime, song, theme},
    enums::LocalizedEnum,
    typesense::{
        documents::{
            HasId,
            anime_document::{AnimeDocument, build_anime_documents},
            song_document::SongDocument,
        },
        index_document::BuildDocumentsFuture,
    },
};

pub const QUERY_BY: &str = "song.title,song.title_native,anime.title,type_sequence,anime.title_english,anime.title_native,anime.synonyms";
pub const QUERY_BY_WEIGHTS: &str = "10,8,6,6,5,5,4";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "themes", enable_nested_fields = true)]
pub struct ThemeDocument {
    pub id: String,
    pub r#type: i32,
    pub sequence: Option<i32>,
    pub type_sequence: String,
    pub anime: AnimeDocument,
    pub song: Option<SongDocument>,
    #[typesense(sort)]
    pub song_title: Option<String>,
    #[typesense(sort)]
    pub created_at: i64,
}

impl HasId for ThemeDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

type ThemeDocumentFrom = (theme::Model, AnimeDocument, Option<song::Model>);

impl From<ThemeDocumentFrom> for ThemeDocument {
    fn from((model, anime_document, song): ThemeDocumentFrom) -> Self {
        Self {
            id: model.id.to_string(),
            r#type: model.r#type.to_value(),
            sequence: model.sequence,
            type_sequence: format!("{}{}", model.r#type.localize(), model.sequence.unwrap_or(1)),
            anime: anime_document,
            song: song.clone().map(SongDocument::from),
            song_title: song.as_ref().and_then(|song| song.title.clone()),
            created_at: model.created_at.timestamp(),
        }
    }
}

pub fn build_theme_documents<'a>(
    models: Vec<theme::Model>,
    database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, ThemeDocument> {
    Box::pin(async move {
        let anime_models: Vec<anime::Model> = models
            .load_one(anime::Entity, database)
            .await?
            .into_iter()
            .map(|anime| anime.expect("Anime not found for theme"))
            .collect();

        let anime_documents = build_anime_documents(anime_models, database).await?;

        let song_models: Vec<Option<song::Model>> = models.load_one(song::Entity, database).await?;

        let documents = models
            .into_iter()
            .zip(anime_documents)
            .zip(song_models)
            .map(|((model, anime_document), song)| {
                ThemeDocument::from((model, anime_document, song))
            })
            .collect();

        Ok(documents)
    })
}
