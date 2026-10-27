use crate::AppError;
use crate::entities::content::entry_videos;
use crate::entities::user::watchhistory;
use crate::rules::validation_error::ValidationError;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, SelectExt,
    TransactionTrait,
};

pub struct MarkAsWatchedActionParameters {
    pub entry_id: u64,
    pub video_id: u64,
    pub user_id: u64,
}

pub struct MarkAsWatchedAction;

impl MarkAsWatchedAction {
    async fn validate(
        db: &DatabaseConnection,
        params: &MarkAsWatchedActionParameters,
    ) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let mut entry_id_errors = Vec::new();
        let mut video_id_errors = Vec::new();

        let entry_video_exists = entry_videos::Entity::find()
            .filter(entry_videos::Column::EntryId.eq(params.entry_id))
            .filter(entry_videos::Column::VideoId.eq(params.video_id))
            .exists(db)
            .await?;

        if !entry_video_exists {
            entry_id_errors.push("The entry video pair does not exist.");
            video_id_errors.push("The entry video pair does not exist.");
        }

        if !entry_id_errors.is_empty() {
            errors.push(ValidationError::new("entry_id", entry_id_errors));
        }

        if !video_id_errors.is_empty() {
            errors.push(ValidationError::new("video_id", video_id_errors));
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn create(
        db: &DatabaseConnection,
        params: MarkAsWatchedActionParameters,
    ) -> Result<watchhistory::Model, AppError> {
        Self::validate(db, &params).await?;

        let txn = db.begin().await?;

        watchhistory::Entity::delete_many()
            .filter(watchhistory::Column::EntryId.eq(params.entry_id))
            .filter(watchhistory::Column::VideoId.eq(params.video_id))
            .filter(watchhistory::Column::UserId.eq(params.user_id))
            .exec(&txn)
            .await?;

        let model = watchhistory::ActiveModel {
            entry_id: Set(params.entry_id),
            video_id: Set(params.video_id),
            user_id: Set(params.user_id),
            ..Default::default()
        };

        let model = model.insert(&txn).await?;

        txn.commit().await?;

        Ok(model)
    }

    pub async fn delete_all(db: &DatabaseConnection, user_id: u64) -> Result<(), AppError> {
        watchhistory::Entity::delete_many()
            .filter(watchhistory::Column::UserId.eq(user_id))
            .exec(db)
            .await?;

        Ok(())
    }
}
