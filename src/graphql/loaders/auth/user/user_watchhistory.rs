use std::collections::HashMap;

use animethemes_server_rust::entities::user::watchhistory;
use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct UserWatchHistoryLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for UserWatchHistoryLoader {
    type Value = Vec<watchhistory::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = watchhistory::Entity::find()
            .filter(watchhistory::Column::UserId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for model in models {
            result.entry(model.user_id).or_default().push(model);
        }

        Ok(result)
    }
}
