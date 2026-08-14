use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::list::track;

pub struct TrackTrackLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for TrackTrackLoader {
    type Value = track::Model;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let tracks = track::Entity::find()
            .filter(track::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db)
            .await?;

        Ok(tracks.into_iter().map(|track| (track.id, track)).collect())
    }
}
