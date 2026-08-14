use std::collections::HashMap;

use animethemes_graphql_rust::entities::auth::{
    model_has_permissions, model_has_roles, permission,
};
use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct UserPermissionsLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for UserPermissionsLoader {
    type Value = Vec<permission::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = model_has_permissions::Entity::find()
            .filter(model_has_roles::Column::ModelId.is_in(keys))
            .find_also_related(permission::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for (pivot, model) in models {
            if let Some(model) = model {
                result.entry(pivot.model_id).or_default().push(model);
            }
        }

        Ok(result)
    }
}
