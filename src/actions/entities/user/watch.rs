use crate::AppError;
use crate::entities::user::watchhistory;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct MarkAsWatchedActionParameters {
    pub entry_id: u64,
    pub video_id: u64,
    pub user_id: u64,
}

pub struct MarkAsWatchedAction;

impl MarkAsWatchedAction {
    pub async fn create(
        db: &DatabaseConnection,
        params: MarkAsWatchedActionParameters,
    ) -> Result<watchhistory::Model, AppError> {
        let model = watchhistory::ActiveModel {
            entry_id: Set(params.entry_id),
            video_id: Set(params.video_id),
            user_id: Set(params.user_id),
            ..Default::default()
        };

        let model = model.insert(db).await?;

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
