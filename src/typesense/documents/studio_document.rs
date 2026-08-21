use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use typesense::Typesense;

use crate::{
    entities::content::studio,
    typesense::{documents::HasId, index_document::BuildDocumentsFuture},
};

pub const QUERY_BY: &str = "name";
pub const QUERY_BY_WEIGHTS: &str = "10";

#[derive(Debug, Clone, Serialize, Deserialize, Typesense)]
#[typesense(collection_name = "studios")]
pub struct StudioDocument {
    pub id: String,
    #[typesense(sort)]
    pub name: String,
    #[typesense(sort)]
    pub created_at: i64,
}

impl HasId for StudioDocument {
    fn id(&self) -> &str {
        &self.id
    }
}

impl From<studio::Model> for StudioDocument {
    fn from(model: studio::Model) -> Self {
        Self {
            id: model.id.to_string(),
            name: model.name.clone(),
            created_at: model.created_at.timestamp(),
        }
    }
}

pub fn build_studio_documents<'a>(
    models: Vec<studio::Model>,
    _database: &'a DatabaseConnection,
) -> BuildDocumentsFuture<'a, StudioDocument> {
    Box::pin(async move {
        let documents = models.into_iter().map(StudioDocument::from).collect();

        Ok(documents)
    })
}
