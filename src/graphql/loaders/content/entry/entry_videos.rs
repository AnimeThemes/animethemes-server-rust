use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;

use crate::entities::content::{entry_videos, video};

pub struct EntryVideosLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for EntryVideosLoader {
    type Value = Vec<(entry_videos::Model, video::Model)>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let rows = entry_videos::Entity::find()
            .filter(entry_videos::Column::EntryId.is_in(keys))
            .find_also_related(video::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for (pivot, model) in rows {
            if let Some(model) = model {
                result
                    .entry(pivot.entry_id)
                    .or_default()
                    .push((pivot, model));
            }
        }

        Ok(result)
    }
}
