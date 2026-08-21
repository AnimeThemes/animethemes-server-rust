use std::collections::HashMap;

use crate::entities::list::playlist;
use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct TrackPlaylistLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for TrackPlaylistLoader {
    type Value = playlist::Model;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = playlist::Entity::find()
            .filter(playlist::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(|model| (model.id, model)).collect())
    }
}
