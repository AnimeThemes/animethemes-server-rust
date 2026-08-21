use std::env;

use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::{entities::SoftDeleteEntity, enums::content::imagefacet::ImageFacet};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "images")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "image_id")]
    pub id: u64,
    pub facet: ImageFacet,
    pub path: String,
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

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

impl ActiveModelBehavior for ActiveModel {}
