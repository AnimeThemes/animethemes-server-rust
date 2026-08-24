use crate::AppError;
use crate::actions::entities::user::watch::{MarkAsWatchedAction, MarkAsWatchedActionParameters};
use async_graphql::{Context, Error, Object, Result};
use sea_orm::DatabaseConnection;

use crate::graphql::types::user::watchhistory::WatchHistory;
use crate::middlewares::current_user::CurrentUser;

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

        let model = MarkAsWatchedAction::create(
            db,
            MarkAsWatchedActionParameters {
                entry_id,
                video_id,
                user_id: user.user.clone().id,
            },
        )
        .await?;

        Ok(model.into())
    }
}
