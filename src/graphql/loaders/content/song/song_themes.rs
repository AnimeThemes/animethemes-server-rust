use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::theme;

pub struct SongThemesLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for SongThemesLoader {
    type Value = Vec<theme::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = theme::Entity::find()
            .filter(theme::Column::SongId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for model in models {
            result
                .entry(model.song_id.unwrap())
                .or_default()
                .push(model);
        }

        Ok(result)
    }
}
