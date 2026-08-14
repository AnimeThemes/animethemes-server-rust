use async_graphql::SimpleObject;

use crate::entities::content::synonym;

/// Represents an alternate title or common abbreviation for an entity.
///
/// For example, the anime Bakemonogatari has the synonym "Monstory".
#[derive(SimpleObject)]
pub struct Synonym {
    /// The primary key of the resource
    pub id: u64,
    /// The language of the synonym. It may be used for short synonyms
    pub language: Option<String>,
    /// The alternate title or common abbreviations
    pub text: String,
}

impl From<synonym::Model> for Synonym {
    fn from(model: synonym::Model) -> Self {
        Self {
            id: model.id,
            language: model.language,
            text: model.text,
        }
    }
}
