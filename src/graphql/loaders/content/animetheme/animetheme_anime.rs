use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::anime;

pub struct AnimeThemeAnimeLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for AnimeThemeAnimeLoader {
    type Value = anime::Model;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let animes = anime::Entity::find()
            .filter(anime::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db)
            .await?;

        Ok(animes.into_iter().map(|anime| (anime.id, anime)).collect())
    }
}
