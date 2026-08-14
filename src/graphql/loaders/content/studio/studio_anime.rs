use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;

use crate::entities::content::{anime, anime_studios};

pub struct StudioAnimeLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for StudioAnimeLoader {
    type Value = Vec<(anime_studios::Model, anime::Model)>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let rows = anime_studios::Entity::find()
            .filter(anime_studios::Column::StudioId.is_in(keys))
            .find_also_related(anime::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for (pivot, anime) in rows {
            if let Some(anime) = anime {
                result
                    .entry(pivot.studio_id)
                    .or_default()
                    .push((pivot, anime));
            }
        }

        Ok(result)
    }
}
