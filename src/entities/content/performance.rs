use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    SoftDeleteEntity,
    content::{artist, song},
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "performances")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "performance_id")]
    pub id: u64,
    pub alias: Option<String>,
    pub r#as: Option<String>,
    pub member_alias: Option<String>,
    pub member_as: Option<String>,
    pub relevance: i32,
    pub artist_id: u64,
    pub member_id: Option<u64>,
    pub song_id: u64,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(belongs_to, relation_enum = "Artist", from = "artist_id", to = "id")]
    pub artist: BelongsTo<artist::Entity>,

    #[sea_orm(belongs_to, relation_enum = "Member", from = "member_id", to = "id")]
    pub member: BelongsTo<Option<artist::Entity>>,

    #[sea_orm(belongs_to, from = "song_id", to = "id")]
    pub song: BelongsTo<song::Entity>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

impl ActiveModelBehavior for ActiveModel {}
