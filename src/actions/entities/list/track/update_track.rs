use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, SelectExt,
    TransactionTrait,
};
use validator::{Validate, ValidationError};

use crate::AppError;
use crate::entities::content::entry_videos;
use crate::entities::list::track;
use crate::traits::sortable::Sortable;

#[derive(InputObject, Validate)]
#[graphql(name = "UpdatePlaylistTrackInput")]
pub struct UpdateTrackActionParameters {
    pub entry_id: Option<u64>,
    pub video_id: Option<u64>,

    #[validate(range(min = 1, message = "The position must be greater than 0."))]
    pub position: Option<i32>,
}

pub struct UpdateTrackAction {}

impl UpdateTrackAction {
    async fn validate(
        db: &DatabaseConnection,
        track: &track::Model,
        params: &UpdateTrackActionParameters,
    ) -> Result<(), AppError> {
        let mut errors = params.validate().err().unwrap_or_default();

        let entry_id = params.entry_id.or(track.entry_id).unwrap();
        let video_id = params.video_id.or(track.video_id).unwrap();

        let exists = entry_videos::Entity::find()
            .filter(entry_videos::Column::EntryId.eq(entry_id))
            .filter(entry_videos::Column::VideoId.eq(video_id))
            .exists(db)
            .await?;

        if !exists {
            let error = ValidationError::new("not_found")
                .with_message("The entry video pair does not exist.".into());

            if params.entry_id.is_some() {
                errors.add("entry_id", error.clone());
            }

            if params.video_id.is_some() {
                errors.add("video_id", error);
            }
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn update(
        ctx: &AppContext,
        track: track::Model,
        params: UpdateTrackActionParameters,
    ) -> Result<track::Model, AppError> {
        Self::validate(&ctx.db, &track, &params).await?;

        let txn = ctx.db.begin().await?;

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
