use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::content::externalresource;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "resourceables")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub r#as: Option<String>,
    pub resource_id: u64,
    pub resourceable_type: String,
    pub resourceable_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(belongs_to, from = "resource_id", to = "id")]
    pub resource: BelongsTo<externalresource::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
