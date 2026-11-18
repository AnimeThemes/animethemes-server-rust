use chrono::Utc;
use macros::SoftDeleteEntity;
use sea_orm::entity::prelude::*;

use crate::entities::content::anime;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, SoftDeleteEntity, DeriveEntityModel)]
#[sea_orm(table_name = "series")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "series_id")]
    pub id: u64,
    pub slug: String,
    pub title: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(has_many, via = "anime_series")]
    pub anime: HasMany<anime::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
