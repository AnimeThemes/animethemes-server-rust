use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    SoftDeleteEntity,
    content::{animetheme, video},
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_theme_entries")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "entry_id")]
    pub id: u64,
    pub theme_id: u64,
    pub episodes: Option<String>,
    pub favorites_count: i32,
    pub notes: Option<String>,
    pub nsfw: bool,
    pub spoiler: bool,
    pub tracks_count: i32,
    pub version: i32,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(belongs_to, from = "theme_id", to = "id")]
    pub theme: BelongsTo<animetheme::Entity>,

    #[sea_orm(has_many, via = "animethemeentry_videos")]
    pub videos: HasMany<video::Entity>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

impl ActiveModelBehavior for ActiveModel {}
