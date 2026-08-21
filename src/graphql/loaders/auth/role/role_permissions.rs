use std::collections::HashMap;

use crate::entities::auth::{model_has_roles, permission, role_has_permissions};
use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct RolePermissionsLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for RolePermissionsLoader {
    type Value = Vec<permission::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = role_has_permissions::Entity::find()
            .filter(model_has_roles::Column::RoleId.is_in(keys))
            .find_also_related(permission::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for (pivot, model) in models {
            if let Some(model) = model {
                result.entry(pivot.role_id).or_default().push(model);
            }
        }

        Ok(result)
    }
}
