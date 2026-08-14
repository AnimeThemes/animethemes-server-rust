use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;

use crate::entities::content::{externalresource, resourceable};

pub struct ResourceableLoader {
    pub db: DatabaseConnection,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceableKey {
    pub resourceable_type: String,
    pub id: u64,
}

impl Loader<ResourceableKey> for ResourceableLoader {
    type Value = Vec<(resourceable::Model, externalresource::Model)>;
    type Error = sea_orm::DbErr;

    async fn load(
        &self,
        keys: &[ResourceableKey],
    ) -> Result<HashMap<ResourceableKey, Self::Value>, Self::Error> {
        let ids: Vec<u64> = keys.iter().map(|k| k.id).collect();

        let types: Vec<String> = keys.iter().map(|k| k.resourceable_type.clone()).collect();

        let rows = resourceable::Entity::find()
            .filter(resourceable::Column::ResourceableType.is_in(types))
            .filter(resourceable::Column::ResourceableId.is_in(ids))
            .find_also_related(externalresource::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<ResourceableKey, Self::Value> = HashMap::new();

        for (pivot, resource) in rows {
            if let Some(resource) = resource {
                let key = ResourceableKey {
                    resourceable_type: pivot.resourceable_type.clone(),
                    id: pivot.resourceable_id,
                };

                result.entry(key).or_default().push((pivot, resource));
            }
        }

        Ok(result)
    }
}
