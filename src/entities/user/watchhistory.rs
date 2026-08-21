use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    auth::user,
    content::{animethemeentry, video},
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "watch_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub entry_id: u64,
    pub video_id: u64,
    pub user_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(belongs_to, from = "entry_id", to = "id")]
    pub entry: BelongsTo<animethemeentry::Entity>,
    #[sea_orm(belongs_to, from = "video_id", to = "id")]
    pub video: BelongsTo<video::Entity>,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
