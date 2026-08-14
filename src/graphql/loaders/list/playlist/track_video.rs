use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::video;

pub struct TrackVideoLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for TrackVideoLoader {
    type Value = video::Model;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let videos = video::Entity::find()
            .filter(video::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db)
            .await?;

        Ok(videos.into_iter().map(|video| (video.id, video)).collect())
    }
}
