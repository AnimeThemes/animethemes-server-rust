use sea_orm::{
    entity::prelude::*,
    sqlx::types::chrono::{self, Utc},
};

use crate::entities::content::artist;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "artist_member")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub artist_id: u64,
    #[sea_orm(primary_key)]
    pub member_id: u64,
    pub alias: Option<String>,
    pub r#as: Option<String>,
    pub notes: Option<String>,
    pub relevance: i32,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(belongs_to, relation_enum = "Artist", from = "artist_id", to = "id")]
    pub artist: BelongsTo<artist::Entity>,

    #[sea_orm(belongs_to, relation_enum = "Member", from = "member_id", to = "id")]
    pub member: BelongsTo<artist::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
