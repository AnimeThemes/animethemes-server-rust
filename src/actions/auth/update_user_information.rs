use loco_rs::validator::ValidateEmail;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, SelectExt,
};

use crate::{
    AppError, actions::auth::verify_email::VerifyEmail, entities::auth::user,
    rules::validation_error::ValidationError,
};

#[derive(Clone)]
pub struct UpdateUserInformationParameters {
    pub name: Option<String>,
    pub email: Option<String>,
}

pub struct UpdateUserInformation;

impl UpdateUserInformation {
    async fn validate(
        db: &DatabaseConnection,
        user: &user::Model,
        params: &UpdateUserInformationParameters,
    ) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let mut name_errors = Vec::new();
        let mut email_errors = Vec::new();

        if let Some(name) = params.name.as_deref() {
            if !(1usize..=35).contains(&name.chars().count()) {
                name_errors.push("The name must be between 1 and 35 characters.");
            }
        }

        if let Some(email) = params.email.as_deref() {
            if !email.validate_email() {
                email_errors.push("The email is not valid.");
            }

            if !(1usize..=255).contains(&email.chars().count()) {
                email_errors.push("The email must be between 1 and 255 characters.");
            }
        }

        let exists = user::Entity::find()
            .filter(
                Condition::any()
                    .add(user::Column::Name.eq(params.name.clone()))
                    .add(user::Column::Email.eq(params.email.clone())),
            )
            .filter(user::Column::Id.ne(user.id))
            .exists(db)
            .await?;

        if exists {
            name_errors.push("User already exists with username or email.");
            email_errors.push("User already exists with username or email.");
        }

        if !name_errors.is_empty() {
            errors.push(ValidationError::new("name", name_errors));
        }

        if !email_errors.is_empty() {
            errors.push(ValidationError::new("email", email_errors));
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn update(
        db: &DatabaseConnection,
        user: user::Model,
        params: UpdateUserInformationParameters,
    ) -> Result<bool, AppError> {
        Self::validate(db, &user, &params).await?;

        let current_email = user.email.clone();

        let mut active_user: user::ActiveModel = user.into();

        if let Some(name) = params.name {
            active_user.name = Set(name);
        }

        let email_has_changed = matches!(
            params.email.as_deref(),
            Some(email) if email != current_email
        );

        if email_has_changed {
            active_user.email = Set(params.email.unwrap());
            active_user.email_verified_at = Set(None);
        }

        let user = active_user.update(db).await?;

        if email_has_changed {
            VerifyEmail::send_verification_email(&user).await?;
        }

        Ok(true)
    }
}
