use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;
use sea_orm::EntityTrait;
use tower_sessions::Session;

use crate::AppError;
use crate::actions::auth::forgot_password::ForgotPassword;
use crate::actions::auth::login::{LoginAction, LoginActionParameters};
use crate::actions::auth::register::{Register, RegisterParameters};
use crate::actions::auth::reset_password::{ResetPassword, ResetPasswordParams};
use crate::actions::auth::update_user_information::{
    UpdateUserInformation, UpdateUserInformationParameters,
};
use crate::actions::auth::update_user_password::{
    UpdateUserPassword, UpdateUserPasswordParameters,
};
use crate::actions::auth::verify_email::VerifyEmail;
use crate::entities::auth::user;
use crate::enums::features::Feature;
use crate::features::functions::FeatureManager;
use crate::graphql::is_development;
use crate::graphql::types::auth::me::Me;
use crate::middlewares::current_user::CurrentUser;

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    #[graphql(visible = "is_development")]
    pub async fn register(&self, ctx: &Context<'_>, input: RegisterParameters) -> Result<Me> {
        let feature_manager = ctx.data_unchecked::<FeatureManager>();

        feature_manager
            .enabled(Feature::Registration, None)
            .extend()?;

        let user = Register::register(ctx.data_unchecked::<AppContext>(), input)
            .await
            .extend()?;

        let session = ctx.data::<Session>()?;

        session.insert("user_id", user.id).await?;

        Ok(Me::from(user))
    }

    #[graphql(visible = "is_development")]
    pub async fn login(&self, ctx: &Context<'_>, input: LoginActionParameters) -> Result<Me> {
        let user = LoginAction::login(ctx.data_unchecked::<AppContext>(), input)
            .await
            .extend()?;

        let session = ctx.data::<Session>()?;

        session.insert("user_id", user.id).await?;

        Ok(user.into())
    }

    #[graphql(visible = "is_development")]
    pub async fn update_user_information(
        &self,
        ctx: &Context<'_>,
        input: UpdateUserInformationParameters,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?;

        let result = UpdateUserInformation::update(
            ctx.data_unchecked::<AppContext>(),
            user.clone(),
            user.user.clone(),
            input,
        )
        .await
        .extend()?;

        Ok(result)
    }

    #[graphql(visible = "is_development")]
    pub async fn update_password(
        &self,
        ctx: &Context<'_>,
        input: UpdateUserPasswordParameters,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?
            .user
            .clone();

        UpdateUserPassword::update(ctx.data_unchecked::<AppContext>(), user, input)
            .await
            .extend()?;

        Ok(true)
    }

    #[graphql(visible = "is_development")]
    pub async fn forgot_password(&self, ctx: &Context<'_>, email: String) -> Result<bool> {
        ForgotPassword::send_reset_password_email(ctx.data_unchecked::<AppContext>(), email)
            .await
            .extend()?;

        Ok(true)
    }

    #[graphql(visible = "is_development")]
    pub async fn reset_password(
        &self,
        ctx: &Context<'_>,
        input: ResetPasswordParams,
    ) -> Result<bool> {
        ResetPassword::reset_password(ctx.data_unchecked::<AppContext>(), input)
            .await
            .extend()?;

        Ok(true)
    }

    #[graphql(visible = "is_development")]
    pub async fn resend_email_verification(&self, ctx: &Context<'_>) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?;

        let user = user::Entity::find_by_id(user.user.id)
            .one(&ctx.data_unchecked::<AppContext>().db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        VerifyEmail::send_verification_email(&user).await.extend()?;

        Ok(true)
    }

    #[graphql(visible = "is_development")]
    pub async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let session = ctx.data::<Session>()?;

        session.delete().await?;

        Ok(true)
    }
}
