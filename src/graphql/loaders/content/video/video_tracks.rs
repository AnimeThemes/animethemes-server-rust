use std::collections::HashMap;

use animethemes_server_rust::{
    entities::list::{playlist, track},
    enums::list::playlistvisibility::PlaylistVisibility,
};
use async_graphql::dataloader::Loader;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};

pub struct VideoTracksLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for VideoTracksLoader {
    type Value = Vec<track::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = track::Entity::find()
            .filter(track::Column::VideoId.is_in(keys))
            .join(JoinType::InnerJoin, track::Relation::Playlist.def())
            .filter(playlist::Column::Visibility.eq(PlaylistVisibility::Public))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for model in models {
            result
                .entry(model.video_id.unwrap())
                .or_default()
                .push(model);
        }

        Ok(result)
    }
}
