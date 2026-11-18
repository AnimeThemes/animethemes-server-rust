use chrono::Utc;
use macros::SoftDeleteEntity;
use sea_orm::entity::prelude::*;

use crate::enums::content::resourcesite::ResourceSite;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, SoftDeleteEntity, DeriveEntityModel)]
#[sea_orm(table_name = "resources")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "resource_id")]
    pub id: u64,
    pub external_id: Option<i32>,
    pub link: String,
    pub site: ResourceSite,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}
