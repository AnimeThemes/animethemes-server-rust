use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::animetheme;

pub struct AnimeThemesLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for AnimeThemesLoader {
    type Value = Vec<animetheme::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = animetheme::Entity::find()
            .filter(animetheme::Column::AnimeId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for model in models {
            result.entry(model.anime_id).or_default().push(model);
        }

        Ok(result)
    }
}
