use std::collections::HashMap;

use animethemes_graphql_rust::entities::list::playlist;
use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct UserPlaylistsLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for UserPlaylistsLoader {
    type Value = Vec<playlist::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let playlists = playlist::Entity::find()
            .filter(playlist::Column::UserId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for playlist in playlists {
            result
                .entry(playlist.user_id.unwrap())
                .or_default()
                .push(playlist);
        }

        Ok(result)
    }
}
