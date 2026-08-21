use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::animethemeentry;

pub struct WatchHistoryEntryLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for WatchHistoryEntryLoader {
    type Value = animethemeentry::Model;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = animethemeentry::Entity::find()
            .filter(animethemeentry::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(|model| (model.id, model)).collect())
    }
}
