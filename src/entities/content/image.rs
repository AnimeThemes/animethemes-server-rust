use std::env;

use chrono::Utc;
use macros::SoftDeleteEntity;
use sea_orm::Condition;
use sea_orm::entity::prelude::*;

use crate::enums::content::imagefacet::ImageFacet;
use crate::traits::sortable::Sortable;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, SoftDeleteEntity, DeriveEntityModel)]
#[sea_orm(table_name = "images")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "image_id")]
    pub id: u64,
    pub facet: ImageFacet,
    pub path: String,
    pub depth: i32,
    pub imageable_type: Option<ImageableType>,
    pub imageable_id: Option<u64>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl Model {
    pub fn link(&self) -> String {
        let image_url = env::var("IMAGE_URL").expect("IMAGE_URL must be set in .env");

        format!("{}/{}", image_url, self.path)
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Sortable for Model {
    fn order_column() -> <Self::Entity as EntityTrait>::Column {
        self::Column::Depth
    }

    fn sort_scope(&self) -> sea_orm::Condition {
        Condition::all()
            .add(self::Column::ImageableType.eq(self.imageable_type))
            .add(self::Column::ImageableId.eq(self.imageable_id))
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(255))")]
pub enum ImageableType {
    #[sea_orm(string_value = "anime")]
    Anime,
    #[sea_orm(string_value = "artist")]
    Artist,
    #[sea_orm(string_value = "studio")]
    Studio,
}
