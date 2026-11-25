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
    pub r#as: Option<String>,
    pub resourceable_type: ResourceableType,
    pub resourceable_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: time::OffsetDateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: time::OffsetDateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<time::OffsetDateTime>,
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
