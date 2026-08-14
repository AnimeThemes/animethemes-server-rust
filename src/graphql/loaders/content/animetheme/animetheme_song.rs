use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::song;

pub struct AnimeThemeSongLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for AnimeThemeSongLoader {
    type Value = song::Model;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = song::Entity::find()
            .filter(song::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(|model| (model.id, model)).collect())
    }
}
