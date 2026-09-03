use crate::entities::content::entry;
use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;

use crate::entities::content::entry_videos;

pub struct VideoThemeEntriesLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for VideoThemeEntriesLoader {
    type Value = Vec<(entry_videos::Model, entry::Model)>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let rows = entry_videos::Entity::find()
            .filter(entry_videos::Column::VideoId.is_in(keys))
            .find_also_related(entry::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for (pivot, model) in rows {
            if let Some(model) = model {
                result
                    .entry(pivot.video_id)
                    .or_default()
                    .push((pivot, model));
            }
        }

        Ok(result)
    }
}
