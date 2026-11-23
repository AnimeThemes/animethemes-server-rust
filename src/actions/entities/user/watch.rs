use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, SelectExt,
    TransactionTrait,
};
use validator::{ValidationError, ValidationErrors};

use crate::AppError;
use crate::entities::content::entry_videos;
use crate::entities::user::watchhistory;

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
        let exists = entry_videos::Entity::find()
            .filter(entry_videos::Column::EntryId.eq(params.entry_id))
            .filter(entry_videos::Column::VideoId.eq(params.video_id))
            .exists(db)
            .await?;

        if !exists {
            let mut errors = ValidationErrors::new();

            let error = ValidationError::new("not_found")
                .with_message("The entry video pair does not exist.".into());

            errors.add("entry_id", error.clone());
            errors.add("video_id", error);

            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn create(
        ctx: &AppContext,
        params: MarkAsWatchedActionParameters,
    ) -> Result<watchhistory::Model, AppError> {
        Self::validate(&ctx.db, &params).await?;

        let txn = ctx.db.begin().await?;

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

    pub async fn delete_all(ctx: &AppContext, user_id: u64) -> Result<(), AppError> {
        watchhistory::Entity::delete_many()
            .filter(watchhistory::Column::UserId.eq(user_id))
            .exec(&ctx.db)
            .await?;

        Ok(())
    }
}
