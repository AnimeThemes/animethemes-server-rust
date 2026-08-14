use sea_orm::{
    entity::prelude::*,
    sqlx::types::chrono::{self, Utc},
};

use crate::entities::content::{anime, studio};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_studio")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub anime_id: u64,
    #[sea_orm(primary_key)]
    pub studio_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(belongs_to, from = "anime_id", to = "id")]
    pub anime: BelongsTo<anime::Entity>,

    #[sea_orm(belongs_to, from = "studio_id", to = "id")]
    pub studio: BelongsTo<studio::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
