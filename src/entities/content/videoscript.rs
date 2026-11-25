use std::env;

use macros::SoftDeleteEntity;
use sea_orm::entity::prelude::*;

use crate::entities::content::video;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, SoftDeleteEntity, DeriveEntityModel)]
#[sea_orm(table_name = "video_scripts")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "script_id")]
    pub id: u64,
    pub video_id: u64,
    pub path: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: time::OffsetDateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: time::OffsetDateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<time::OffsetDateTime>,

    #[sea_orm(belongs_to, from = "video_id", to = "id")]
    pub video: BelongsTo<video::Entity>,
}

impl Model {
    pub fn link(&self) -> String {
        let script_url = env::var("SCRIPT_URL").expect("SCRIPT_URL must be set in .env");

        format!("{}/{}", script_url, self.id)
    }
}

impl ActiveModelBehavior for ActiveModel {}
