use bcrypt::hash;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, ModelTrait, QueryFilter,
};
use sha2::{Digest, Sha256};

use crate::{
    AppError,
    actions::auth::register::Register,
    entities::auth::{password_reset_tokens, user},
    rules::validation_error::ValidationError,
};

pub struct ResetPasswordParams {
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
    pub token: String,
}

pub struct ResetPassword;

impl ResetPassword {
    fn hash_token(token: &str) -> String {
        hex::encode(Sha256::digest(token.as_bytes()))
    }

    fn validate(params: &ResetPasswordParams) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let password_errors =
            Register::get_password_errors(&params.password, &params.password_confirmation);

        if !password_errors.is_empty() {
            errors.push(ValidationError::new("password", password_errors));
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn reset_password(
        db: &DatabaseConnection,
        params: ResetPasswordParams,
    ) -> Result<(), AppError> {
        Self::validate(&params)?;

        let token = password_reset_tokens::Entity::find()
            .filter(password_reset_tokens::Column::Email.eq(params.email.clone()))
            .filter(password_reset_tokens::Column::Token.eq(Self::hash_token(&params.token)))
            .one(db)
            .await?;

        if let Some(token) = token {
            let user = user::Entity::find_by_email(params.email).one(db).await?;

            if let Some(user) = user {
                let mut active_model = user.into_active_model();

                active_model.password = Set(hash(params.password, 12).map_err(AppError::internal)?);
                active_model.updated_at = Set(chrono::Utc::now());
                active_model.save(db).await?;

                token.delete(db).await?;

                return Ok(());
            }
        }

        Err(AppError::Forbidden)
    }
}
