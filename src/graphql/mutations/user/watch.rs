use animethemes_server_rust::entities::user::watchhistory;
use async_graphql::{Context, Error, Object, Result};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::graphql::types::user::watchhistory::WatchHistory;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::AppError;

#[derive(Default)]
pub struct WatchMutation;

#[Object]
impl WatchMutation {
    /// Mark a video as watched.
    async fn watch(&self, ctx: &Context<'_>, entry_id: u64, video_id: u64) -> Result<WatchHistory> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let model = watchhistory::ActiveModel {
            entry_id: Set(entry_id),
            video_id: Set(video_id),
            user_id: Set(user.user.clone().id),
            ..Default::default()
        };

        let model = model.insert(db).await?;

        Ok(model.into())
    }
}
