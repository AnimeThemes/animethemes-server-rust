use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::audio;

pub struct VideoAudioLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for VideoAudioLoader {
    type Value = audio::Model;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let audios = audio::Entity::find()
            .filter(audio::Column::Id.is_in(keys.iter().copied()))
            .all(&self.db)
            .await?;

        Ok(audios.into_iter().map(|audio| (audio.id, audio)).collect())
    }
}
