use crate::entities::content::{artist, artist_members};
use async_graphql::dataloader::Loader;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use std::collections::HashMap;

pub struct ArtistGroupsLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for ArtistGroupsLoader {
    type Value = Vec<(artist_members::Model, artist::Model)>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let rows = artist_members::Entity::find()
            .filter(artist_members::Column::MemberId.is_in(keys))
            .join(JoinType::LeftJoin, artist_members::Relation::Artist.def())
            .select_also(artist::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for (pivot, model) in rows {
            if let Some(model) = model {
                result
                    .entry(pivot.member_id)
                    .or_default()
                    .push((pivot, model));
            }
        }

        Ok(result)
    }
}
