use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::animethemeentry;

pub struct TrackEntryLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for TrackEntryLoader {
    type Value = animethemeentry::Model;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let entries = animethemeentry::Entity::find()
            .filter(animethemeentry::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db)
            .await?;

        Ok(entries.into_iter().map(|entry| (entry.id, entry)).collect())
    }
}
