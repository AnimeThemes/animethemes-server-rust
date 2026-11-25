use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "feature_flags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: String,
    pub scope_type: String,
    pub scope_id: Option<u64>,
    pub value: String,
    #[serde(default = "crate::entities::default_datetime")]
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: time::OffsetDateTime,
    #[serde(default = "crate::entities::default_datetime")]
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: time::OffsetDateTime,
}

impl ActiveModelBehavior for ActiveModel {}
