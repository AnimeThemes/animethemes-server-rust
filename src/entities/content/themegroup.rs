use chrono::Utc;
use macros::SoftDeleteEntity;
use sea_orm::entity::prelude::*;

use crate::entities::content::theme;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, SoftDeleteEntity, DeriveEntityModel)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "group_id")]
    pub id: u64,
    pub name: String,
    pub slug: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(has_many)]
    pub themes: HasMany<theme::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
