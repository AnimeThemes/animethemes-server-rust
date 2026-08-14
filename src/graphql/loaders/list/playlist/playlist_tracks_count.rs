use std::collections::HashMap;

use animethemes_server_rust::entities::list::track;
use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

pub struct PlaylistTracksCountLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for PlaylistTracksCountLoader {
    type Value = i64;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let tracks: Vec<(u64, i64)> = track::Entity::find()
            .select_only()
            .column(track::Column::PlaylistId)
            .column_as(track::Column::Id.count(), "count")
            .filter(track::Column::PlaylistId.is_in(keys.iter().copied()))
            .group_by(track::Column::PlaylistId)
            .into_tuple()
            .all(&self.db)
            .await?;

        Ok(tracks.into_iter().collect())
    }
}
