use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;

use crate::entities::content::{animethemeentry_videos, video};

pub struct AnimeThemeEntryVideosLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for AnimeThemeEntryVideosLoader {
    type Value = Vec<(animethemeentry_videos::Model, video::Model)>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let rows = animethemeentry_videos::Entity::find()
            .filter(animethemeentry_videos::Column::EntryId.is_in(keys))
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
