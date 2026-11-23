use async_graphql::InputObject;
use bcrypt::{hash, verify};
use loco_rs::app::AppContext;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use validator::{Validate, ValidationError};

use crate::AppError;
use crate::entities::auth::user;
use crate::rules::*;

#[derive(InputObject, Clone, Validate)]
#[graphql(name = "UpdatePasswordInput")]
pub struct UpdateUserPasswordParameters {
    #[graphql(secret)]
    pub current_password: String,

    #[graphql(secret)]
    #[validate(length(
        min = 8,
        max = 72,
        message = "The password must be between 8 and 72 characters."
    ))]
    #[validate(custom(
        function = "chars::has_uppercase",
        message = "The password must contain at least one uppercase letter."
    ))]
    #[validate(custom(
        function = "chars::has_lowercase",
        message = "The password must contain at least one lowercase letter."
    ))]
    #[validate(custom(
        function = "chars::has_number",
        message = "The password must contain at least one number."
    ))]
    #[validate(custom(
        function = "chars::has_symbol",
        message = "The password must contain at least one symbol."
    ))]
    #[validate(must_match(
        other = "new_password_confirmation",
        message = "The password confirmation does not match."
    ))]
    pub new_password: String,

    #[graphql(secret)]
    pub new_password_confirmation: String,
}

pub struct UpdateUserPassword;

impl UpdateUserPassword {
    async fn validate(
        user: &user::Model,
        params: &UpdateUserPasswordParameters,
    ) -> Result<(), AppError> {
        let mut errors = params.validate().err().unwrap_or_default();

        if !verify(&params.current_password, &user.password).map_err(AppError::internal)? {
            errors.add(
                "current_password",
                ValidationError::new("invalid_password")
                    .with_message("Invalid current password".into()),
            );
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn update(
        ctx: &AppContext,
        user: user::Model,
        params: UpdateUserPasswordParameters,
    ) -> Result<bool, AppError> {
        Self::validate(&user, &params).await?;

        let mut user: user::ActiveModel = user.clone().into();

        user.password = Set(hash(&params.new_password, 12).map_err(AppError::internal)?);

        user.update(&ctx.db).await?;

        Ok(true)
    }
}
