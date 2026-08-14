use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::{
    entities::{
        SoftDeleteEntity,
        content::{animetheme, series, studio, synonym},
    },
    enums::content::{animeformat::AnimeFormat, animeseason::AnimeSeason},
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "anime_id")]
    pub id: u64,
    pub format: Option<AnimeFormat>,
    pub season: Option<AnimeSeason>,
    pub slug: String,
    pub synopsis: Option<String>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_native: Option<String>,
    pub year: Option<i32>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(
        has_many,
        // Unsupported
        //on_condition = r#"synonym::Column::SynonymableType.eq("anime")"#
    )]
    pub synonyms: HasMany<synonym::Entity>,

    #[sea_orm(has_many)]
    pub animethemes: HasMany<animetheme::Entity>,

    #[sea_orm(has_many, via = "anime_series")]
    pub series: HasMany<series::Entity>,

    #[sea_orm(has_many, via = "anime_studios")]
    pub studios: HasMany<studio::Entity>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

impl ActiveModelBehavior for ActiveModel {}
