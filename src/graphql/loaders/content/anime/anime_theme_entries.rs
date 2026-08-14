use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::animethemeentry;

pub struct AnimeThemeEntriesLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for AnimeThemeEntriesLoader {
    type Value = Vec<animethemeentry::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = animethemeentry::Entity::find()
            .filter(animethemeentry::Column::ThemeId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for model in models {
            result.entry(model.theme_id).or_default().push(model);
        }

        Ok(result)
    }
}
