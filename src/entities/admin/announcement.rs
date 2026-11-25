use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "announcements")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "announcement_id")]
    pub id: u64,
    pub content: String,
    #[sea_orm(column_type = "Timestamp")]
    pub start_at: Option<time::OffsetDateTime>,
    #[sea_orm(column_type = "Timestamp")]
    pub end_at: Option<time::OffsetDateTime>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: time::OffsetDateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: time::OffsetDateTime,
}

impl ActiveModelBehavior for ActiveModel {}
