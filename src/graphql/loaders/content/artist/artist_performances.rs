use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::performance;

pub struct ArtistPerformancesLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for ArtistPerformancesLoader {
    type Value = Vec<performance::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let performances = performance::Entity::find()
            .filter(performance::Column::ArtistId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for performance in performances {
            result
                .entry(performance.artist_id)
                .or_default()
                .push(performance);
        }

        Ok(result)
    }
}
