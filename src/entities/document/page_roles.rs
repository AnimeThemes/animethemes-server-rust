use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::{
    entities::{auth::role, document::page},
    enums::document::pageroletype::PageRoleType,
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "page_roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub page_id: u64,
    #[sea_orm(primary_key)]
    pub role_id: u64,
    pub r#type: PageRoleType,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(belongs_to, from = "page_id", to = "id")]
    pub page: BelongsTo<page::Entity>,

    #[sea_orm(belongs_to, from = "role_id", to = "id")]
    pub role: BelongsTo<role::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
