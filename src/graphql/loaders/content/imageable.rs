use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;

use crate::entities::content::{image, imageable};

pub struct ImageableLoader {
    pub db: DatabaseConnection,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImageableKey {
    pub imageable_type: String,
    pub id: u64,
}

impl Loader<ImageableKey> for ImageableLoader {
    type Value = Vec<(imageable::Model, image::Model)>;
    type Error = sea_orm::DbErr;

    async fn load(
        &self,
        keys: &[ImageableKey],
    ) -> Result<HashMap<ImageableKey, Self::Value>, Self::Error> {
        let ids: Vec<u64> = keys.iter().map(|k| k.id).collect();

        let types: Vec<String> = keys.iter().map(|k| k.imageable_type.clone()).collect();

        let rows = imageable::Entity::find()
            .filter(imageable::Column::ImageableType.is_in(types))
            .filter(imageable::Column::ImageableId.is_in(ids))
            .find_also_related(image::Entity)
            .all(&self.db)
            .await?;

        let mut result: HashMap<ImageableKey, Self::Value> = HashMap::new();

        for (pivot, model) in rows {
            if let Some(model) = model {
                let key = ImageableKey {
                    imageable_type: pivot.imageable_type.clone(),
                    id: pivot.imageable_id,
                };

                result.entry(key).or_default().push((pivot, model));
            }
        }

        Ok(result)
    }
}
