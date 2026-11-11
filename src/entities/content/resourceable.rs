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
    pub resourceable_type: ResourceableType,
    pub resourceable_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(belongs_to, from = "resource_id", to = "id")]
    pub resource: BelongsTo<externalresource::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Copy, Clone, Eq, Hash, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(255))")]
pub enum ResourceableType {
    #[sea_orm(string_value = "anime")]
    Anime,
    #[sea_orm(string_value = "artist")]
    Artist,
    #[sea_orm(string_value = "entry")]
    Entry,
    #[sea_orm(string_value = "song")]
    Song,
    #[sea_orm(string_value = "studio")]
    Studio,
}
