use std::collections::HashMap;

use crate::{
    entities::user::rating,
    graphql::{
        enums::sort::{GraphQLSort, user::rating_sort::RatingSort},
        loaders::group_by_query,
    },
};
use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Select};

pub struct UserRatingsLoader {
    pub db: DatabaseConnection,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct UserRatingsLoaderQuery {
    pub sort: Option<Vec<RatingSort>>,
}

impl UserRatingsLoaderQuery {
    fn sort(&self, mut query: Select<rating::Entity>) -> Select<rating::Entity> {
        if let Some(sorts) = &self.sort {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        query
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct UserRatingsLoaderKey {
    pub key: u64,
    pub query: UserRatingsLoaderQuery,
}

impl Loader<UserRatingsLoaderKey> for UserRatingsLoader {
    type Value = Vec<rating::Model>;
    type Error = sea_orm::DbErr;

    async fn load(
        &self,
        keys: &[UserRatingsLoaderKey],
    ) -> Result<HashMap<UserRatingsLoaderKey, Self::Value>, Self::Error> {
        let groups = group_by_query(keys, |key| key.query.clone());

        let mut result: HashMap<UserRatingsLoaderKey, Self::Value> =
            keys.iter().cloned().map(|key| (key, Vec::new())).collect();

        for (query, group_keys) in groups {
            let ids = group_keys.iter().map(|key| key.key).collect::<Vec<_>>();

            let builder = rating::Entity::find().filter(rating::Column::UserId.is_in(ids));

            let builder = query.sort(builder);

            let models = builder.all(&self.db).await?;

            let keys_by_id: HashMap<u64, &UserRatingsLoaderKey> =
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
