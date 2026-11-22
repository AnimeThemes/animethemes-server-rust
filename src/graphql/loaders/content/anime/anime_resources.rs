use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::resource;
use crate::scopes::without_trashed;

pub struct AnimeResourcesLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for AnimeResourcesLoader {
    type Value = Vec<resource::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = resource::Entity::find()
            .filter(without_trashed::<resource::Entity>())
            .filter(resource::Column::ResourceableType.eq(resource::ResourceableType::Anime))
            .filter(resource::Column::ResourceableId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for model in models {
            result.entry(model.resourceable_id).or_default().push(model);
        }

        Ok(result)
    }
}
