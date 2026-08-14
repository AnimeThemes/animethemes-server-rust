use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::{
    entities::{
        SoftDeleteEntity,
        content::{anime, animethemeentry, song, themegroup},
    },
    enums::content::themetype::ThemeType,
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_themes")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "theme_id")]
    pub id: u64,
    pub anime_id: u64,
    pub group_id: Option<u64>,
    pub sequence: Option<i32>,
    pub slug: String,
    pub song_id: Option<u64>,
    pub r#type: ThemeType,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(belongs_to, from = "anime_id", to = "id")]
    pub anime: BelongsTo<anime::Entity>,

    #[sea_orm(belongs_to, from = "song_id", to = "id")]
    pub song: BelongsTo<Option<song::Entity>>,

    #[sea_orm(belongs_to, from = "group_id", to = "id")]
    pub theme_group: BelongsTo<Option<themegroup::Entity>>,

    #[sea_orm(has_many)]
    pub animethemeentries: HasMany<animethemeentry::Entity>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

impl ActiveModelBehavior for ActiveModel {}
