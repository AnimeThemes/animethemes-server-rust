use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    SoftDeleteEntity,
    content::{song_staff, synonym},
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "artists")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "artist_id")]
    pub id: u64,
    pub slug: String,
    pub name: String,
    pub name_native: Option<String>,
    pub information: Option<String>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(
        has_many,
        // Unsupported
        //on_condition = r#"synonym::Column::SynonymableType.eq("artist")"#
    )]
    pub synonyms: HasMany<synonym::Entity>,

    #[sea_orm(has_many, relation_enum = "SongStaff", via_rel = "Artist")]
    pub song_staff: HasMany<song_staff::Entity>,

    #[sea_orm(has_many, relation_enum = "MemberSongStaff", via_rel = "Member")]
    pub member_song_staff: HasMany<song_staff::Entity>,

    #[sea_orm(self_ref, via = "artist_members", from = "Artist", to = "Member")]
    pub members: HasMany<Entity>,

    #[sea_orm(self_ref, via = "artist_members", reverse)]
    pub groups: HasMany<Entity>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

impl ActiveModelBehavior for ActiveModel {}
