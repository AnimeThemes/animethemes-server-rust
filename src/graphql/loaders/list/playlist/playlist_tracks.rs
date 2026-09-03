use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Select};

use crate::{
    entities::list::track,
    graphql::{
        enums::sort::{GraphQLSort, list::track_sort::PlaylistTrackSort},
        loaders::group_by_query,
        types::list::playlist::PlaylistTracksFilterInput,
    },
};

pub struct PlaylistTracksLoader {
    pub db: DatabaseConnection,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct PlaylistTracksLoaderQuery {
    pub filter: Option<PlaylistTracksFilterInput>,
    pub sort: Option<Vec<PlaylistTrackSort>>,
}

impl PlaylistTracksLoaderQuery {
    fn condition(&self) -> Condition {
        let mut condition = Condition::all()
            .add(track::Column::EntryId.is_not_null())
            .add(track::Column::VideoId.is_not_null());

        if let Some(filter) = &self.filter {
            if let Some(entry_id) = filter.entry_id.clone() {
                condition = condition.add(track::Column::EntryId.eq(entry_id));
            }

            if let Some(video_id) = filter.video_id.clone() {
                condition = condition.add(track::Column::VideoId.eq(video_id));
            }
        }

        condition
    }

    fn sort(&self, mut query: Select<track::Entity>) -> Select<track::Entity> {
        if let Some(sorts) = &self.sort {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        query
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct PlaylistTracksLoaderKey {
    pub key: u64,
    pub query: PlaylistTracksLoaderQuery,
}

impl Loader<PlaylistTracksLoaderKey> for PlaylistTracksLoader {
    type Value = Vec<track::Model>;
    type Error = sea_orm::DbErr;

    async fn load(
        &self,
        keys: &[PlaylistTracksLoaderKey],
    ) -> Result<HashMap<PlaylistTracksLoaderKey, Self::Value>, Self::Error> {
        let groups = group_by_query(keys, |key| key.query.clone());

        let mut result: HashMap<PlaylistTracksLoaderKey, Self::Value> =
            keys.iter().cloned().map(|key| (key, Vec::new())).collect();

        for (query, group_keys) in groups {
            let ids = group_keys.iter().map(|key| key.key).collect::<Vec<_>>();

            let builder = track::Entity::find()
                .filter(track::Column::PlaylistId.is_in(ids))
                .filter(query.condition());

            let builder = query.sort(builder);

            let models = builder.all(&self.db).await?;

            let keys_by_id: HashMap<u64, &PlaylistTracksLoaderKey> =
                group_keys.into_iter().map(|key| (key.key, key)).collect();

            for model in models {
                if let Some(key) = keys_by_id.get(&model.playlist_id).copied() {
                    result
                        .get_mut(key)
                        .expect("loader key must exist in result")
                        .push(model);
                }
            }
        }

        Ok(result)
    }
}
