use crate::entities::content::animethemeentry;
use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;

use crate::entities::content::animethemeentry_videos;

pub struct VideoAnimeThemeEntriesLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for VideoAnimeThemeEntriesLoader {
    type Value = Vec<(animethemeentry_videos::Model, animethemeentry::Model)>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let rows = animethemeentry_videos::Entity::find()
            .filter(animethemeentry_videos::Column::VideoId.is_in(keys))
            .find_also_related(animethemeentry::Entity)
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
