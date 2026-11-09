use std::collections::HashMap;

use crate::{
    entities::user::favorite,
    graphql::{
        enums::sort::{GraphQLSort, user::favorite_sort::FavoriteSort},
        loaders::group_by_query,
        types::auth::me::UserFavoritesFilterInput,
    },
};
use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Select};

pub struct UserFavoritesLoader {
    pub db: DatabaseConnection,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct UserFavoritesLoaderQuery {
    pub filter: Option<UserFavoritesFilterInput>,
    pub sort: Option<Vec<FavoriteSort>>,
}

impl UserFavoritesLoaderQuery {
    fn condition(&self) -> Condition {
        let mut condition = Condition::all();

        if let Some(filter) = &self.filter {
            if let Some(entry_id) = filter.entry_id.clone() {
                condition = condition
                    .add(favorite::Column::FavoriteableId.eq(entry_id))
                    .add(favorite::Column::FavoriteableType.eq("entry"));
            }
        }

        condition
    }

    fn sort(&self, mut query: Select<favorite::Entity>) -> Select<favorite::Entity> {
        if let Some(sorts) = &self.sort {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        query
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct UserFavoritesLoaderKey {
    pub key: u64,
    pub query: UserFavoritesLoaderQuery,
}

impl Loader<UserFavoritesLoaderKey> for UserFavoritesLoader {
    type Value = Vec<favorite::Model>;
    type Error = sea_orm::DbErr;

    async fn load(
        &self,
        keys: &[UserFavoritesLoaderKey],
    ) -> Result<HashMap<UserFavoritesLoaderKey, Self::Value>, Self::Error> {
        let groups = group_by_query(keys, |key| key.query.clone());

        let mut result: HashMap<UserFavoritesLoaderKey, Self::Value> =
            keys.iter().cloned().map(|key| (key, Vec::new())).collect();

        for (query, group_keys) in groups {
            let ids = group_keys.iter().map(|key| key.key).collect::<Vec<_>>();

            let builder = favorite::Entity::find()
                .filter(favorite::Column::UserId.is_in(ids))
                .filter(query.condition());

            let builder = query.sort(builder).order_by_id_desc();

            let models = builder.all(&self.db).await?;

            let keys_by_id: HashMap<u64, &UserFavoritesLoaderKey> =
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
