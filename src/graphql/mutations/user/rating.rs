use async_graphql::{Context, Error, InputObject, Object, Result, ResultExt};
use loco_rs::app::AppContext;

use crate::AppError;
use crate::actions::entities::user::rating::{RateEntryActionParameters, RatingAction};
use crate::graphql::types::user::rating::Rating;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject)]
struct RateEntryInput {
    /// The score to rate the entry with.
    pub score: Option<f32>,
}

#[derive(Default)]
pub struct RatingMutation;

#[Object]
impl RatingMutation {
    // Rate an entry for the authenticated user.
    async fn rate_entry(
        &self,
        ctx: &Context<'_>,
        entry_id: u64,
        input: RateEntryInput,
    ) -> Result<Option<Rating>> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?;

        let model = RatingAction::rate_entry(
            ctx.data_unchecked::<AppContext>(),
            RateEntryActionParameters {
                entry_id,
                score: input.score,
                user_id: user.user.clone().id,
            },
        )
        .await
        .extend()?;

        Ok(model.map(Into::into))
    }

    /// Clear the ratings for the authenticated user.
    async fn clear_ratings(&self, ctx: &Context<'_>) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?;

        RatingAction::delete_all(ctx.data_unchecked::<AppContext>(), user.user.id)
            .await
            .extend()?;

        Ok(true)
    }
}
