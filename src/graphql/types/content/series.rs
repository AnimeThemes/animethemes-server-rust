use async_graphql::SimpleObject;

use crate::entities::content::series;

#[derive(SimpleObject)]
pub struct SeriesTitle {
    romaji: String,
}

impl From<&series::Model> for SeriesTitle {
    fn from(model: &series::Model) -> Self {
        Self {
            romaji: model.title.clone(),
        }
    }
}

/// Represents a collection of related anime.
///
/// For example, the Monogatari series is the collection of the Bakemonogatari anime and its related productions.
#[derive(SimpleObject)]
pub struct Series {
    /// The primary key of the resource
    pub id: u64,
    /// The primary title of the series
    pub title: SeriesTitle,
    /// The URL slug & route key of the resource
    pub slug: String,
}

impl From<series::Model> for Series {
    fn from(model: series::Model) -> Self {
        let title = SeriesTitle::from(&model);
        Self {
            id: model.id,
            slug: model.slug,
            title,
        }
    }
}
