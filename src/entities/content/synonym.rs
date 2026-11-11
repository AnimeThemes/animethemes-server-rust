use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::SoftDeleteEntity;
use crate::entities::content::{anime, artist};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "synonyms")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "synonym_id")]
    pub id: u64,
    pub language: Option<String>,
    pub synonymable_type: SynonymableType,
    pub synonymable_id: u64,
    pub text: String,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(belongs_to, from = "synonymable_id", to = "id")]
    pub anime: BelongsTo<anime::Entity>,

    #[sea_orm(belongs_to, from = "synonymable_id", to = "id")]
    pub artist: BelongsTo<artist::Entity>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Copy, Clone, Eq, Hash, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(255))")]
pub enum SynonymableType {
    #[sea_orm(string_value = "anime")]
    Anime,
    #[sea_orm(string_value = "artist")]
    Artist,
}
