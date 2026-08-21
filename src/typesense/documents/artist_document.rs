use std::collections::HashMap;

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::{artist, performance, synonym},
    typesense::{documents::HasId, index_document::BuildDocumentsFuture},
};

pub const QUERY_BY: &str = "name,name_native,synonyms,as,search_text";
pub const QUERY_BY_WEIGHTS: &str = "10,8,6,3,1";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "artists")]
pub struct ArtistDocument {
    pub id: String,
    #[typesense(sort)]
    pub name: String,
    #[typesense(sort)]
    pub name_native: Option<String>,
    pub created_at: i64,
    pub r#as: Vec<String>,
    pub synonyms: Vec<String>,
    pub search_text: String,
}

impl HasId for ArtistDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

type ArtistDocumentFrom = (
    artist::Model,
    Vec<synonym::Model>,
    Vec<performance::Model>,
    Vec<performance::Model>,
);

impl From<ArtistDocumentFrom> for ArtistDocument {
    fn from((model, synonyms, performances, member_performances): ArtistDocumentFrom) -> Self {
        let r#as = performances
            .iter()
            .filter_map(|p| p.r#as.clone())
            .chain(
                member_performances
                    .iter()
                    .filter_map(|p| p.member_as.clone()),
            )
            .collect::<Vec<String>>();

        let name_native = if model.name_native.as_ref() == Some(&model.name) {
            None
        } else {
            model.name_native.clone()
        };

        let mut search_text = vec![
            model.name.clone(),
            synonyms.iter().map(|s| s.text.clone()).collect(),
        ];

        if let Some(name_native) = name_native.clone() {
            search_text.push(name_native);
        }

        Self {
            id: model.id.to_string(),
            name: model.name.clone(),
            name_native: name_native,
            created_at: model.created_at.timestamp(),
            r#as: r#as,
            synonyms: synonyms.iter().map(|s| s.text.clone()).collect(),
            search_text: search_text.join(" "),
        }
    }
}

pub fn build_artist_documents<'a>(
    models: Vec<artist::Model>,
    database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, ArtistDocument> {
    Box::pin(async move {
        let synonyms: Vec<Vec<synonym::Model>> = models
            .load_many(
                synonym::Entity::find().filter(synonym::Column::SynonymableType.eq("artist")),
                database,
            )
            .await?;

        let artist_ids: Vec<u64> = models.iter().map(|model| model.id).collect();

        let performances = performance::Entity::find()
            .filter(performance::Column::ArtistId.is_in(artist_ids.clone()))
            .all(database)
            .await?;

        let member_performances = performance::Entity::find()
            .filter(performance::Column::MemberId.is_in(artist_ids))
            .all(database)
            .await?;

        let mut performances_by_artist: HashMap<u64, Vec<performance::Model>> = HashMap::new();

        for performance in performances {
            performances_by_artist
                .entry(performance.artist_id)
                .or_default()
                .push(performance);
        }

        let mut performances_by_member: HashMap<u64, Vec<performance::Model>> = HashMap::new();

        for performance in member_performances {
            if let Some(member_id) = performance.member_id {
                performances_by_member
                    .entry(member_id)
                    .or_default()
                    .push(performance);
            }
        }

        let documents = models
            .into_iter()
            .zip(synonyms)
            .map(|(model, synonyms)| {
                let performances = performances_by_artist.remove(&model.id).unwrap_or_default();

                let member_performances =
                    performances_by_member.remove(&model.id).unwrap_or_default();

                ArtistDocument::from((model, synonyms, performances, member_performances))
            })
            .collect();

        Ok(documents)
    })
}
