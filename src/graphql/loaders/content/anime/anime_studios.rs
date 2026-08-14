use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;

use crate::entities::content::{anime_studios, studio};

pub struct AnimeStudiosLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for AnimeStudiosLoader {
    type Value = Vec<(anime_studios::Model, studio::Model)>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let rows = anime_studios::Entity::find()
            .filter(anime_studios::Column::AnimeId.is_in(keys))
            .find_also_related(studio::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for (pivot, model) in rows {
            if let Some(model) = model {
                result
                    .entry(pivot.anime_id)
                    .or_default()
                    .push((pivot, model));
            }
        }

        Ok(result)
    }
}
