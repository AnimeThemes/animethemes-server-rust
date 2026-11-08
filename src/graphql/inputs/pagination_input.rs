use async_graphql::{Context, InputObject};

use crate::{
    AppError, middlewares::rate_limiting::CanBypass, rules::validation_error::ValidationError,
};

#[derive(InputObject, Default)]
pub struct PaginationInput {
    pub after: Option<String>,
    pub before: Option<String>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

impl PaginationInput {
    pub fn validate(&self, ctx: &Context<'_>) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let can_bypass = ctx.data_unchecked::<CanBypass>().0;

        if can_bypass {
            return Ok(());
        }

        if let Some(first) = self.first {
            if !(1..=100).contains(&first) {
                errors.push(ValidationError::new(
                    "first",
                    vec!["The first must be between 1 and 100".to_string()],
                ))
            }
        }

        if let Some(last) = self.last {
            if !(1..=100).contains(&last) {
                errors.push(ValidationError::new(
                    "last",
                    vec!["The last must be between 1 and 100".to_string()],
                ))
            }
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }
}
