use crate::AppError;
use crate::actions::entities::user::watch::{MarkAsWatchedAction, MarkAsWatchedActionParameters};
use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;

use crate::graphql::types::user::watchhistory::WatchHistory;
use crate::middlewares::current_user::CurrentUser;

#[derive(Default)]
pub struct WatchMutation;

#[Object]
impl WatchMutation {
    /// Mark a video as watched for the authenticated user.
    async fn watch(&self, ctx: &Context<'_>, entry_id: u64, video_id: u64) -> Result<WatchHistory> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?;

        let model = MarkAsWatchedAction::create(
            ctx.data_unchecked::<AppContext>(),
            MarkAsWatchedActionParameters {
                entry_id,
                video_id,
                user_id: user.user.clone().id,
            },
        )
        .await
        .extend()?;

        Ok(model.into())
    }

    /// Clear the watch history for the authenticated user.
    async fn clear_watch_history(&self, ctx: &Context<'_>) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?;

        MarkAsWatchedAction::delete_all(ctx.data_unchecked::<AppContext>(), user.user.clone().id)
            .await
            .extend()?;

        Ok(true)
    }
}
