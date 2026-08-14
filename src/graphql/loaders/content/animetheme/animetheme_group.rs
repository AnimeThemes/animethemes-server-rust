use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::themegroup;

pub struct AnimeThemeGroupLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for AnimeThemeGroupLoader {
    type Value = themegroup::Model;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let groups = themegroup::Entity::find()
            .filter(themegroup::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db)
            .await?;

        Ok(groups.into_iter().map(|group| (group.id, group)).collect())
    }
}
