use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::content::image;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "imageables")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub depth: i32,
    pub image_id: u64,
    pub imageable_type: String,
    pub imageable_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(belongs_to, from = "image_id", to = "id")]
    pub image: BelongsTo<image::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
