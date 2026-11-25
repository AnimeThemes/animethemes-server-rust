use std::env;

use macros::SoftDeleteEntity;
use sea_orm::entity::prelude::*;

use crate::entities::content::video;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, SoftDeleteEntity, DeriveEntityModel)]
#[sea_orm(table_name = "audios")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "audio_id")]
    pub id: u64,
    pub basename: String,
    pub filename: String,
    pub mimetype: String,
    pub path: String,
    pub size: i32,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: time::OffsetDateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: time::OffsetDateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<time::OffsetDateTime>,

    #[sea_orm(has_many, from = "id", to = "audio_id")]
    pub videos: HasMany<video::Entity>,
}

impl Model {
    pub fn link(&self) -> String {
        let audio_url = env::var("AUDIO_URL").expect("AUDIO_URL must be set in .env");

        format!("{}/{}", audio_url, self.basename)
    }
}

impl ActiveModelBehavior for ActiveModel {}
