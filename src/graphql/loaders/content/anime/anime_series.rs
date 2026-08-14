use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;

use crate::entities::content::{anime_series, series};

pub struct AnimeSeriesLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for AnimeSeriesLoader {
    type Value = Vec<(anime_series::Model, series::Model)>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let rows = anime_series::Entity::find()
            .filter(anime_series::Column::AnimeId.is_in(keys))
            .find_also_related(series::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for (pivot, series) in rows {
            if let Some(series) = series {
                result
                    .entry(pivot.anime_id)
                    .or_default()
                    .push((pivot, series));
            }
        }

        Ok(result)
    }
}
