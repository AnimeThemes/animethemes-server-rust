use sea_orm::{ActiveEnum, DatabaseConnection, LoaderTrait};
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::{anime, animetheme, song},
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

pub const QUERY_BY: &str = "song.title,song.title_native,anime.title,type_sequence, anime.title_english,anime.title_native,anime.synonyms";
pub const QUERY_BY_WEIGHTS: &str = "10,8,6,6,5,5,4";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "animethemes", enable_nested_fields = true)]
pub struct AnimeThemeDocument {
    pub id: String,
    pub r#type: i32,
    pub sequence: Option<i32>,
    pub type_sequence: String,
    pub anime: AnimeDocument,
    pub song: Option<SongDocument>,
    pub created_at: i64,
}

impl HasId for AnimeThemeDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

type AnimeThemeDocumentFrom = (animetheme::Model, AnimeDocument, Option<song::Model>);

impl From<AnimeThemeDocumentFrom> for AnimeThemeDocument {
    fn from((model, anime_document, song): AnimeThemeDocumentFrom) -> Self {
        Self {
            id: model.id.to_string(),
            r#type: model.r#type.to_value(),
            sequence: model.sequence,
            type_sequence: format!("{}{}", model.r#type.localize(), model.sequence.unwrap_or(1)),
            anime: anime_document,
            song: song.map(SongDocument::from),
            created_at: model.created_at.timestamp(),
        }
    }
}

pub fn build_animetheme_documents<'a>(
    models: Vec<animetheme::Model>,
    database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, AnimeThemeDocument> {
    Box::pin(async move {
        let anime_models: Vec<anime::Model> = models
            .load_one(anime::Entity, database)
            .await?
            .into_iter()
            .map(|anime| anime.expect("Anime not found for animetheme"))
            .collect();

        let anime_documents = build_anime_documents(anime_models, database).await?;

        let song_models: Vec<Option<song::Model>> = models.load_one(song::Entity, database).await?;

        let documents = models
            .into_iter()
            .zip(anime_documents)
            .zip(song_models)
            .map(|((model, anime_document), song)| {
                AnimeThemeDocument::from((model, anime_document, song))
            })
            .collect();

        Ok(documents)
    })
}
