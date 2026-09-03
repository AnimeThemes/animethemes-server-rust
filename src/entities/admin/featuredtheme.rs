use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    auth::user,
    content::{entry, video},
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "featured_themes")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "featured_theme_id")]
    pub id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub start_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub end_at: Option<chrono::DateTime<Utc>>,
    pub entry_id: Option<u64>,
    pub video_id: Option<u64>,
    pub user_id: Option<u64>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(belongs_to, from = "entry_id", to = "id")]
    pub entry: BelongsTo<Option<entry::Entity>>,

    #[sea_orm(belongs_to, from = "video_id", to = "id")]
    pub video: BelongsTo<Option<video::Entity>>,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<Option<user::Entity>>,
}

impl ActiveModelBehavior for ActiveModel {}
