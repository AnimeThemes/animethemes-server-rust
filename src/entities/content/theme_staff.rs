use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    SoftDeleteEntity,
    content::{artist, theme},
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "theme_staff")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: u64,
    pub alias: Option<String>,
    pub relevance: i32,
    pub role: String,
    pub artist_id: u64,
    pub theme_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(belongs_to, from = "artist_id", to = "id")]
    pub artist: BelongsTo<artist::Entity>,

    #[sea_orm(belongs_to, from = "theme_id", to = "id")]
    pub theme: BelongsTo<theme::Entity>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

impl ActiveModelBehavior for ActiveModel {}
