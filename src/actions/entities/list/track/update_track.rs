use sea_orm::{ActiveModelTrait, DatabaseConnection, TransactionTrait};

use crate::{
    AppError, entities::list::track, rules::validation_error::ValidationError,
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
    fn validate(params: &UpdateTrackActionParameters) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let mut position_errors = Vec::new();

        if let Some(position) = params.position {
            if position < 1 {
                position_errors.push("The position must be greater than 0.");
            }
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
        Self::validate(&params)?;

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
