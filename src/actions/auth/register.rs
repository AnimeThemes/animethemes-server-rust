use async_graphql::InputObject;
use bcrypt::hash;
use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
    SelectExt,
};
use validator::{Validate, ValidationError};

use crate::entities::auth::user::{self};
use crate::events::auth::user_registered::UserRegistered;
use crate::rules::*;
use crate::{AppError, events};

#[derive(InputObject, Clone, Validate)]
#[graphql(name = "RegisterInput")]
pub struct RegisterParameters {
    #[validate(length(
        min = 1,
        max = 35,
        message = "The name must be between 1 and 35 characters."
    ))]
    #[validate(custom(
        function = "alpha_dash::validate_alpha_dash",
        message = "The name must be alphanumeric with dashes or underscores."
    ))]
    pub name: String,

    #[validate(email(message = "The email is not valid."))]
    #[validate(length(
        min = 1,
        max = 255,
        message = "The email must be between 1 and 255 characters."
    ))]
    #[validate(custom(
        function = "validate_non_disposable_email",
        message = "Disposable email addresses are not allowed."
    ))]
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

    #[validate(custom(
        function = "bool::validate_bool_true_required",
        message = "You must accept the terms and conditions."
    ))]
    pub terms: bool,
}

pub struct Register;

impl Register {
    async fn validate(
        db: &DatabaseConnection,
        params: &RegisterParameters,
    ) -> Result<(), AppError> {
        let mut errors = params.validate().err().unwrap_or_default();

        let exists = user::Entity::find()
            .filter(
                Condition::any()
                    .add(user::Column::Name.eq(params.name.clone()))
                    .add(user::Column::Email.eq(params.email.clone())),
            )
            .exists(db)
            .await?;

        if exists {
            let error = ValidationError::new("already_exists")
                .with_message("User already exists with username or email.".into());

            errors.add("name", error.clone());
            errors.add("email", error);
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn register(
        ctx: &AppContext,
        params: RegisterParameters,
    ) -> Result<user::Model, AppError> {
        Self::validate(&ctx.db, &params).await?;

        let model = user::ActiveModel {
            name: Set(params.name.clone()),
            email: Set(params.email.clone()),
            password: Set(hash(params.password, 12).map_err(AppError::internal)?),
            ..Default::default()
        };

        let user = model.insert(&ctx.db).await?;

        events::dispatch(
            ctx,
            UserRegistered {
                user: user.clone(),
                auth_user: None,
            },
        )
        .await;

        Ok(user)
    }
}

fn validate_non_disposable_email(email: &str) -> Result<(), ValidationError> {
    if disposable::is_email(email) {
        Err(ValidationError::new("disposable_email"))
    } else {
        Ok(())
    }
}
