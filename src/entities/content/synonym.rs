use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    SoftDeleteEntity,
    content::{anime, artist},
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "synonyms")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "synonym_id")]
    pub id: u64,
    pub language: Option<String>,
    pub synonymable_type: String,
    pub synonymable_id: u64,
    pub text: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(has_many)]
    pub anime: HasMany<anime::Entity>,

    #[sea_orm(has_many)]
    pub artists: HasMany<artist::Entity>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

impl ActiveModelBehavior for ActiveModel {}
