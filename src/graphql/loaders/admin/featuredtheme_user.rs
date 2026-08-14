use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::auth::user;

pub struct FeaturedThemeUserLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for FeaturedThemeUserLoader {
    type Value = user::Model;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let users = user::Entity::find()
            .filter(user::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db)
            .await?;

        Ok(users.into_iter().map(|user| (user.id, user)).collect())
    }
}
