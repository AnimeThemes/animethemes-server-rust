use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
    SelectExt,
};
use validator::{Validate, ValidationError};

use crate::actions::auth::verify_email::VerifyEmail;
use crate::entities::auth::user;
use crate::events::auth::user_updated::UserUpdated;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::*;
use crate::{AppError, events};

#[derive(InputObject, Clone, Validate)]
#[graphql(name = "UpdateUserInformationInput")]
pub struct UpdateUserInformationParameters {
    #[validate(length(
        min = 1,
        max = 35,
        message = "The name must be between 1 and 35 characters."
    ))]
    #[validate(custom(
        function = "alpha_dash::validate_alpha_dash",
        message = "The name must be alphanumeric with dashes or underscores."
    ))]
    pub name: Option<String>,

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
    pub email: Option<String>,
}

pub struct UpdateUserInformation;

impl UpdateUserInformation {
    async fn validate(
        db: &DatabaseConnection,
        user: &user::Model,
        params: &UpdateUserInformationParameters,
    ) -> Result<(), AppError> {
        let mut errors = params.validate().err().unwrap_or_default();

        if params.name.is_some() || params.email.is_some() {
            let exists = user::Entity::find()
                .filter(
                    Condition::any()
                        .add_option(
                            params
                                .name
                                .as_deref()
                                .map(|name| user::Column::Name.eq(name)),
                        )
                        .add_option(
                            params
                                .email
                                .as_deref()
                                .map(|email| user::Column::Email.eq(email)),
                        ),
                )
                .filter(user::Column::Id.ne(user.id))
                .exists(db)
                .await?;

            if exists {
                let error = ValidationError::new("already_exists")
                    .with_message("User already exists with username or email.".into());

                errors.add("name", error.clone());
                errors.add("email", error);
            }
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn update(
        ctx: &AppContext,
        auth_user: CurrentUser,
        user: user::Model,
        params: UpdateUserInformationParameters,
    ) -> Result<bool, AppError> {
        Self::validate(&ctx.db, &user, &params).await?;

        let old_user = user.clone();

        let mut active_user: user::ActiveModel = user.into();

        if let Some(name) = params.name {
            active_user.name.set_ne(name);
        }

        if let Some(email) = params.email.clone() {
            active_user.email.set_ne(email);
        }

        let email_has_changed = active_user.get(user::Column::Email).is_set();

        if email_has_changed {
            active_user.email_verified_at.set_ne(None);
        }

        let user = active_user.update(&ctx.db).await?;

        events::dispatch(
            ctx,
            UserUpdated {
                old_user,
                new_user: user.clone(),
                auth_user: Some(auth_user),
            },
        )
        .await;

        if email_has_changed {
            VerifyEmail::send_verification_email(&user).await?;
        }

        Ok(true)
    }
}

fn validate_non_disposable_email(email: &str) -> Result<(), ValidationError> {
    if disposable::is_email(email) {
        Err(ValidationError::new("disposable_email"))
    } else {
        Ok(())
    }
}
