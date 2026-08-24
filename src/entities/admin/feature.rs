use chrono::Utc;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "feature_flags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: String,
    pub scope_type: String,
    pub scope_id: Option<u64>,
    pub value: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}
