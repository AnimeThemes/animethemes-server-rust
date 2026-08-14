use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::animetheme;

pub struct AnimeThemeEntryThemeLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for AnimeThemeEntryThemeLoader {
    type Value = animetheme::Model;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let themes = animetheme::Entity::find()
            .filter(animetheme::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db)
            .await?;

        Ok(themes.into_iter().map(|theme| (theme.id, theme)).collect())
    }
}
