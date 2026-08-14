use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::performance;

pub struct ArtistMemberPerformancesLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for ArtistMemberPerformancesLoader {
    type Value = Vec<performance::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = performance::Entity::find()
            .filter(performance::Column::MemberId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for model in models {
            result
                .entry(model.member_id.unwrap())
                .or_default()
                .push(model);
        }

        Ok(result)
    }
}
