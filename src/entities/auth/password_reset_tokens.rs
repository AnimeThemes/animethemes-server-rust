use chrono::Utc;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "password_reset_tokens_rust")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub email: String,
    pub token: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}
