use bcrypt::{hash, verify};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

use crate::{AppError, entities::auth::user, rules::validation_error::ValidationError};

#[derive(Clone)]
pub struct UpdateUserPasswordParameters {
    pub current_password: String,
    pub new_password: String,
    pub new_password_confirmation: String,
}

pub struct UpdateUserPassword;

impl UpdateUserPassword {
    pub async fn validate(
        user: &user::Model,
        params: &UpdateUserPasswordParameters,
    ) -> Result<(), AppError> {
        let mut errors = Vec::new();

        if !verify(&params.current_password, &user.password).map_err(AppError::internal)? {
            errors.push(ValidationError::new(
                "current_password",
                vec!["Invalid current password"],
            ));
        }

        let mut password_errors = Vec::new();

        if params.new_password.chars().count() < 8 {
            password_errors.push("The password must be at least 8 characters.");
        }

        if !params.new_password.chars().any(|c| c.is_uppercase()) {
            password_errors.push("The password must contain at least one uppercase letter.");
        }

        if !params.new_password.chars().any(|c| c.is_lowercase()) {
            password_errors.push("The password must contain at least one lowercase letter.");
        }

        if !params.new_password.chars().any(|c| c.is_alphabetic()) {
            password_errors.push("The password must contain at least one letter.");
        }

        if !params.new_password.chars().any(|c| c.is_numeric()) {
            password_errors.push("The password must contain at least one number.");
        }

        if !params
            .new_password
            .chars()
            .any(|c| c.is_ascii_punctuation())
        {
            password_errors.push("The password must contain at least one symbol.");
        }

        if params.new_password != params.new_password_confirmation {
            password_errors.push("The password confirmation does not match.");
        }

        if !password_errors.is_empty() {
            errors.push(ValidationError::new("new_password", password_errors));
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn update(
        db: &DatabaseConnection,
        user: user::Model,
        params: UpdateUserPasswordParameters,
    ) -> Result<bool, AppError> {
        Self::validate(&user, &params).await?;

        let mut user: user::ActiveModel = user.clone().into();

        user.password = Set(hash(&params.new_password, 12).map_err(AppError::internal)?);

        user.update(db).await?;

        Ok(true)
    }
}
