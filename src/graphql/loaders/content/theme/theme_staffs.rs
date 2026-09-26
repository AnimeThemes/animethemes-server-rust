use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::theme_staff;

pub struct ThemeStaffsLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for ThemeStaffsLoader {
    type Value = Vec<theme_staff::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = theme_staff::Entity::find()
            .filter(theme_staff::Column::ThemeId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for model in models {
            result.entry(model.theme_id).or_default().push(model);
        }

        Ok(result)
    }
}
