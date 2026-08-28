use std::collections::HashMap;

use crate::entities::auth::{role, user_roles};
use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct UserRolesLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for UserRolesLoader {
    type Value = Vec<role::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let roles = user_roles::Entity::find()
            .filter(user_roles::Column::UserId.is_in(keys))
            .find_also_related(role::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for (pivot, role) in roles {
            if let Some(role) = role {
                result.entry(pivot.user_id).or_default().push(role);
            }
        }

        Ok(result)
    }
}
