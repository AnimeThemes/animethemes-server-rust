use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, SelectExt,
    TransactionTrait,
};

use crate::{
    AppError,
    entities::{content::entry_videos, list::track},
    rules::validation_error::ValidationError,
    traits::sortable::Sortable,
};
use sea_orm::ActiveValue::Set;

pub struct UpdateTrackActionParameters {
    pub entry_id: Option<u64>,
    pub video_id: Option<u64>,
    pub position: Option<i32>,
}

pub struct UpdateTrackAction {}

impl UpdateTrackAction {
    async fn validate(
        db: &DatabaseConnection,
        track: &track::Model,
        params: &UpdateTrackActionParameters,
    ) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let mut entry_id_errors = Vec::new();
        let mut video_id_errors = Vec::new();
        let mut position_errors = Vec::new();

        let entry_id = params.entry_id.unwrap_or(track.entry_id.unwrap());
        let video_id = params.video_id.unwrap_or(track.video_id.unwrap());

        let entry_video_exists = entry_videos::Entity::find()
            .filter(entry_videos::Column::EntryId.eq(entry_id))
            .filter(entry_videos::Column::VideoId.eq(video_id))
            .exists(db)
            .await?;

        if !entry_video_exists {
            if params.entry_id.is_some() {
                entry_id_errors.push("The entry video pair does not exist.");
            }

            if params.video_id.is_some() {
                video_id_errors.push("The entry video pair does not exist.");
            }
        }

        if let Some(position) = params.position {
            if position < 1 {
                position_errors.push("The position must be greater than 0.");
            }
        }

        if !entry_id_errors.is_empty() {
            errors.push(ValidationError::new("entry_id", entry_id_errors));
        }

        if !video_id_errors.is_empty() {
            errors.push(ValidationError::new("video_id", video_id_errors));
        }

        if !position_errors.is_empty() {
            errors.push(ValidationError::new("position", position_errors));
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn update(
        db: &DatabaseConnection,
        track: track::Model,
        params: UpdateTrackActionParameters,
    ) -> Result<track::Model, AppError> {
        Self::validate(db, &track, &params).await?;

        let txn = db.begin().await?;

        let mut active_model: track::ActiveModel = track.clone().into();

        if let Some(entry_id) = params.entry_id {
            active_model.entry_id = Set(Some(entry_id));
        }

        if let Some(video_id) = params.video_id {
            active_model.video_id = Set(Some(video_id));
        }

        let mut track = active_model.update(&txn).await?;

        if let Some(position) = params.position {
            track = track.move_to(&txn, position).await?;
        }

        txn.commit().await?;

        Ok(track)
    }
}
