use sea_orm::{
    entity::prelude::*,
    sqlx::types::chrono::{self, Utc},
};

use crate::entities::content::{animethemeentry, video};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_theme_entry_video")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub entry_id: u64,
    #[sea_orm(primary_key)]
    pub video_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(belongs_to, from = "entry_id", to = "id")]
    pub animethemeentry: BelongsTo<animethemeentry::Entity>,

    #[sea_orm(belongs_to, from = "video_id", to = "id")]
    pub video: BelongsTo<video::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
