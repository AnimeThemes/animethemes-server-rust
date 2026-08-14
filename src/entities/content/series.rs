use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{SoftDeleteEntity, content::anime};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "series")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "series_id")]
    pub id: u64,
    pub slug: String,
    pub title: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(has_many, via = "anime_series")]
    pub anime: HasMany<anime::Entity>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

impl ActiveModelBehavior for ActiveModel {}
