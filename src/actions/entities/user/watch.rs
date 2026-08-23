use crate::entities::user::watchhistory;
use anyhow::Result;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

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
    ) -> Result<watchhistory::Model> {
        let model = watchhistory::ActiveModel {
            entry_id: Set(params.entry_id),
            video_id: Set(params.video_id),
            user_id: Set(params.user_id),
            ..Default::default()
        };

        let model = model.insert(db).await?;

        Ok(model)
    }
}
