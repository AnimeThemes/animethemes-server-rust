use async_graphql::{Context, InputObject};
use validator::Validate;

use crate::AppError;
use crate::middlewares::rate_limiting::CanBypass;

#[derive(InputObject, Default, Validate)]
pub struct PaginationInput {
    pub after: Option<String>,
    pub before: Option<String>,

    #[validate(range(min = 1, max = 100, message = "The first must be between 1 and 100"))]
    pub first: Option<i32>,

    #[validate(range(min = 1, max = 100, message = "The last must be between 1 and 100"))]
    pub last: Option<i32>,
}

impl PaginationInput {
    pub fn validate(&self, ctx: &Context<'_>) -> Result<(), AppError> {
        if ctx.data_unchecked::<CanBypass>().0 {
            return Ok(());
        }

        Validate::validate(self).map_err(AppError::Validation)
    }
}
