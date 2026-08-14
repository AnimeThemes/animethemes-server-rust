use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::videoscript;

pub struct VideoScriptLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for VideoScriptLoader {
    type Value = videoscript::Model;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let scripts = videoscript::Entity::find()
            .filter(videoscript::Column::VideoId.is_in(keys.iter().copied()))
            .all(&self.db)
            .await?;

        Ok(scripts
            .into_iter()
            .map(|script| (script.id, script))
            .collect())
    }
}
