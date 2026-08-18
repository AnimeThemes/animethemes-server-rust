use std::collections::HashMap;

use animethemes_server_rust::entities::list::playlist;
use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Select};

use crate::graphql::{
    enums::sort::{GraphQLSort, list::playlist_sort::PlaylistSort},
    loaders::group_by_query,
};

pub struct UserPlaylistsLoader {
    pub db: DatabaseConnection,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct UserPlaylistsLoaderQuery {
    pub sort: Option<Vec<PlaylistSort>>,
}

impl UserPlaylistsLoaderQuery {
    fn sort(&self, mut query: Select<playlist::Entity>) -> Select<playlist::Entity> {
        if let Some(sorts) = &self.sort {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        query
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct UserPlaylistsLoaderKey {
    pub key: u64,
    pub query: UserPlaylistsLoaderQuery,
}

impl UserPlaylistsLoaderKey {
    pub fn new(key: u64, sort: Option<Vec<PlaylistSort>>) -> Self {
        Self {
            key,
            query: UserPlaylistsLoaderQuery { sort },
        }
    }
}

impl Loader<UserPlaylistsLoaderKey> for UserPlaylistsLoader {
    type Value = Vec<playlist::Model>;
    type Error = sea_orm::DbErr;

    async fn load(
        &self,
        keys: &[UserPlaylistsLoaderKey],
    ) -> Result<HashMap<UserPlaylistsLoaderKey, Self::Value>, Self::Error> {
        let groups = group_by_query(keys, |key| key.query.clone());

        let mut result: HashMap<UserPlaylistsLoaderKey, Self::Value> =
            keys.iter().cloned().map(|key| (key, Vec::new())).collect();

        for (query, group_keys) in groups {
            let ids = group_keys.iter().map(|key| key.key).collect::<Vec<_>>();

            let builder = playlist::Entity::find().filter(playlist::Column::UserId.is_in(ids));

            let builder = query.sort(builder);

            let models = builder.all(&self.db).await?;

            let keys_by_id: HashMap<u64, &UserPlaylistsLoaderKey> =
                group_keys.into_iter().map(|key| (key.key, key)).collect();

            for model in models {
                if let Some(key) = keys_by_id.get(&model.user_id).copied() {
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
