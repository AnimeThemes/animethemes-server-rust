use async_graphql::InputObject;
use bcrypt::hash;
use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter,
};
use sha2::{Digest, Sha256};
use validator::Validate;

use crate::AppError;
use crate::entities::auth::{password_reset_tokens, user};
use crate::rules::*;

#[derive(InputObject, Validate)]
#[graphql(name = "ResetPasswordInput")]
pub struct ResetPasswordParams {
    pub email: String,

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
        other = "password_confirmation",
        message = "The password confirmation does not match."
    ))]
    pub password: String,

    #[graphql(secret)]
    pub password_confirmation: String,

    #[graphql(secret)]
    pub token: String,
}

pub struct ResetPassword;

impl ResetPassword {
    fn hash_token(token: &str) -> String {
        hex::encode(Sha256::digest(token.as_bytes()))
    }

    pub async fn reset_password(
        ctx: &AppContext,
        params: ResetPasswordParams,
    ) -> Result<(), AppError> {
        if let Err(errors) = params.validate() {
            return Err(AppError::Validation(errors));
        }

        let token = password_reset_tokens::Entity::find()
            .filter(password_reset_tokens::Column::Email.eq(params.email.clone()))
            .filter(password_reset_tokens::Column::Token.eq(Self::hash_token(&params.token)))
            .one(&ctx.db)
            .await?;

        if let Some(token) = token {
            let user = user::Entity::find_by_email(params.email)
                .one(&ctx.db)
                .await?;

            if let Some(user) = user {
                let mut active_model = user.into_active_model();

                active_model.password = Set(hash(params.password, 12).map_err(AppError::internal)?);
                active_model.updated_at = Set(chrono::Utc::now());
                active_model.save(&ctx.db).await?;

                token.delete(&ctx.db).await?;

                return Ok(());
            }
        }

        Err(AppError::Forbidden)
    }
}
