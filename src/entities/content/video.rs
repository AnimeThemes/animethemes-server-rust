use std::env;

use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::{
    entities::{
        SoftDeleteEntity,
        content::{animethemeentry, audio, videoscript},
    },
    enums::{
        LocalizedEnum,
        content::{videooverlap::VideoOverlap, videosource::VideoSource},
    },
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "videos")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "video_id")]
    pub id: u64,
    pub audio_id: Option<u64>,
    pub basename: String,
    pub filename: String,
    pub lyrics: bool,
    pub mimetype: String,
    pub nc: bool,
    pub overlap: VideoOverlap,
    pub path: String,
    pub resolution: Option<i32>,
    pub size: Option<i32>,
    pub source: Option<VideoSource>,
    pub subbed: bool,
    pub uncen: bool,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: chrono::DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(belongs_to, from = "audio_id", to = "id")]
    pub audio: BelongsTo<Option<audio::Entity>>,

    #[sea_orm(has_one)]
    pub video_script: HasOne<videoscript::Entity>,

    #[sea_orm(has_many, via = "animethemeentry_videos")]
    pub animethemeentries: HasMany<animethemeentry::Entity>,
}

impl Model {
    pub fn link(&self) -> String {
        let video_url = env::var("VIDEO_URL").expect("VIDEO_URL must be set in .env");

        format!("{}/{}", video_url, self.basename)
    }

    pub fn tags(&self) -> String {
        let mut tags: Vec<String> = Vec::new();

        if self.nc {
            tags.push("NC".to_string());
        }

        if let Some(source) = self.source {
            if matches!(source, VideoSource::BD | VideoSource::DVD) {
                tags.push(source.localize().to_string());
            }
        }

        if let Some(resolution) = self.resolution {
            if resolution != 720 {
                tags.push(resolution.to_string());
            }
        }

        if self.subbed {
            tags.push("Subbed".to_string());
        } else if self.lyrics {
            tags.push("Lyrics".to_string());
        }

        tags.join("")
    }
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

impl ActiveModelBehavior for ActiveModel {}
