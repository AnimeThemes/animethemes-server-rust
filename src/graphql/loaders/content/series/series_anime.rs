use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;

use crate::entities::content::{anime, anime_series};

pub struct SeriesAnimeLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for SeriesAnimeLoader {
    type Value = Vec<(anime_series::Model, anime::Model)>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let rows = anime_series::Entity::find()
            .filter(anime_series::Column::SeriesId.is_in(keys))
            .find_also_related(anime::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for (pivot, model) in rows {
            if let Some(model) = model {
                result
                    .entry(pivot.series_id)
                    .or_default()
                    .push((pivot, model));
            }
        }

        Ok(result)
    }
}
