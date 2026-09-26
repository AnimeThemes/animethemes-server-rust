use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::song_staff;

pub struct ArtistSongStaffsLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for ArtistSongStaffsLoader {
    type Value = Vec<song_staff::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = song_staff::Entity::find()
            .filter(song_staff::Column::ArtistId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for model in models {
            result.entry(model.artist_id).or_default().push(model);
        }

        Ok(result)
    }
}
