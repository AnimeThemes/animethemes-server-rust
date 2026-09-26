use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::song_staff;

pub struct SongPerformancesLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for SongPerformancesLoader {
    type Value = Vec<song_staff::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = song_staff::Entity::find()
            .filter(song_staff::Column::SongId.is_in(keys))
            .filter(song_staff::Column::Role.eq("Performance"))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for model in models {
            result.entry(model.song_id).or_default().push(model);
        }

        Ok(result)
    }
}
