use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::synonym;

pub struct AnimeSynonymsLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for AnimeSynonymsLoader {
    type Value = Vec<synonym::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = synonym::Entity::find()
            .filter(synonym::Column::SynonymableType.eq("anime"))
            .filter(synonym::Column::SynonymableId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for model in models {
            result.entry(model.synonymable_id).or_default().push(model);
        }

        Ok(result)
    }
}
