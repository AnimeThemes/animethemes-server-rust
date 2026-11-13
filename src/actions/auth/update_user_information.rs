use loco_rs::app::AppContext;
use loco_rs::prelude::BackgroundWorker;
use loco_rs::validator::ValidateEmail;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
    SelectExt,
};

use crate::actions::auth::is_alpha_dash;
use crate::actions::auth::verify_email::VerifyEmail;
use crate::entities::auth::user;
use crate::enums::features::Feature;
use crate::features::functions::FeatureManager;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::validation_error::ValidationError;
use crate::workers::discord_notification::{
    DiscordNotificationWorker, DiscordNotificationWorkerArgs,
};
use crate::{AppError, discord};

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

            if !is_alpha_dash(&name) {
                name_errors.push("The name must be alphanumeric with dashes or underscores.");
            }
        }

        if let Some(email) = params.email.as_deref() {
            if !email.validate_email() {
                email_errors.push("The email is not valid.");
            }

            if disposable::is_email(email) {
                email_errors.push("Disposable email addresses are not allowed.");
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

        if email_has_changed {
            VerifyEmail::send_verification_email(&user).await?;
        }

        let feature_manager = ctx.shared_store.get::<FeatureManager>();

        if let Some(feature_manager) = feature_manager
            && feature_manager.is_enabled(Feature::AllowDiscordNotifications, None, false)
        {
            DiscordNotificationWorker::perform_later(
                ctx,
                DiscordNotificationWorkerArgs {
                    channel_id: discord::user::User::get_discord_channel(),
                    message: serde_json::to_value(
                        discord::user::User::new(old_user, Some(auth_user))
                            .updated_message(user)
                            .ok(),
                    )
                    .unwrap(),
                },
            )
            .await
            .map_err(AppError::internal)?;
        }

        Ok(true)
    }
}
