use crate::entities::content::{artist, artist_members};
use async_graphql::dataloader::Loader;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use std::collections::HashMap;

pub struct ArtistMembersLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for ArtistMembersLoader {
    type Value = Vec<(artist_members::Model, artist::Model)>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let rows = artist_members::Entity::find()
            .filter(artist_members::Column::ArtistId.is_in(keys))
            .join(JoinType::LeftJoin, artist_members::Relation::Member.def())
            .select_also(artist::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for (pivot, model) in rows {
            if let Some(model) = model {
                result
                    .entry(pivot.artist_id)
                    .or_default()
                    .push((pivot, model));
            }
        }

        Ok(result)
    }
}
