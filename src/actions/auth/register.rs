use bcrypt::hash;
use loco_rs::validator::ValidateEmail;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, SelectExt,
};

use crate::{
    AppError,
    actions::auth::verify_email::VerifyEmail,
    entities::auth::user::{self},
    rules::validation_error::ValidationError,
};

#[derive(Clone)]
pub struct CreateUserParameters {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
    pub terms: bool,
}

pub struct Register;

impl Register {
    pub fn get_password_errors<'a>(
        password: &'a String,
        password_confirmation: &'a String,
    ) -> Vec<&'a str> {
        let mut errors = Vec::new();

        if !(8..=72).contains(&password.chars().count()) {
            errors.push("The password must be between 8 and 72 characters.");
        }

        if !password.chars().any(|c| c.is_uppercase()) {
            errors.push("The password must contain at least one uppercase letter.");
        }

        if !password.chars().any(|c| c.is_lowercase()) {
            errors.push("The password must contain at least one lowercase letter.");
        }

        if !password.chars().any(|c| c.is_alphabetic()) {
            errors.push("The password must contain at least one letter.");
        }

        if !password.chars().any(|c| c.is_numeric()) {
            errors.push("The password must contain at least one number.");
        }

        if !password.chars().any(|c| c.is_ascii_punctuation()) {
            errors.push("The password must contain at least one symbol.");
        }

        if password != password_confirmation {
            errors.push("The password confirmation does not match.");
        }

        errors
    }

    async fn validate(
        db: &DatabaseConnection,
        params: &CreateUserParameters,
    ) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let mut name_errors = Vec::new();
        let mut email_errors = Vec::new();
        let password_errors =
            Self::get_password_errors(&params.password, &params.password_confirmation);

        if !(1usize..=35).contains(&params.name.chars().count()) {
            name_errors.push("The name must be between 1 and 35 characters.");
        }

        if !&params.email.validate_email() {
            email_errors.push("The email is not valid.");
        }

        if !(1usize..=255).contains(&params.email.chars().count()) {
            email_errors.push("The email must be between 1 and 255 characters.");
        }

        if !params.terms {
            errors.push(ValidationError::new(
                "terms",
                vec!["You must accept the terms and conditions."],
            ));
        }

        if !password_errors.is_empty() {
            errors.push(ValidationError::new("password", password_errors));
        }

        let exists = user::Entity::find()
            .filter(
                Condition::any()
                    .add(user::Column::Name.eq(params.name.clone()))
                    .add(user::Column::Email.eq(params.email.clone())),
            )
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

    pub async fn register(
        db: &DatabaseConnection,
        params: CreateUserParameters,
    ) -> Result<user::Model, AppError> {
        Self::validate(db, &params).await?;

        let model = user::ActiveModel {
            name: Set(params.name.clone()),
            email: Set(params.email.clone()),
            password: Set(hash(params.password, 12).map_err(AppError::internal)?),
            ..Default::default()
        };

        let user = model.insert(db).await?;

        VerifyEmail::send_verification_email(&user).await?;

        Ok(user)
    }
}
