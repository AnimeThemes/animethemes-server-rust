use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::content::image;
use crate::scopes::without_trashed;

pub struct StudioImagesLoader {
    pub db: DatabaseConnection,
}

impl Loader<u64> for StudioImagesLoader {
    type Value = Vec<image::Model>;
    type Error = sea_orm::DbErr;

    async fn load(&self, keys: &[u64]) -> Result<HashMap<u64, Self::Value>, Self::Error> {
        let models = image::Entity::find()
            .filter(without_trashed::<image::Entity>())
            .filter(image::Column::ImageableType.eq(image::ImageableType::Studio))
            .filter(image::Column::ImageableId.is_in(keys))
            .all(&self.db)
            .await?;

        let mut result: HashMap<u64, Self::Value> = HashMap::new();

        for model in models {
            result
                .entry(model.imageable_id.unwrap_or_default())
                .or_default()
                .push(model);
        }

        Ok(result)
    }
}
