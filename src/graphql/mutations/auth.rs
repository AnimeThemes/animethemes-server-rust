use crate::{
    AppError,
    actions::auth::{
        register::{CreateUserParameters, Register},
        update_user_information::{UpdateUserInformation, UpdateUserInformationParameters},
        update_user_password::{UpdateUserPassword, UpdateUserPasswordParameters},
    },
    entities::auth::user,
};
use async_graphql::{Context, Error, InputObject, Object, Result, ResultExt};
use bcrypt::verify;
use sea_orm::DatabaseConnection;
use tower_sessions::Session;

use crate::{graphql::types::auth::me::Me, middlewares::current_user::CurrentUser};

#[derive(InputObject)]
pub struct RegisterInput {
    name: String,
    #[graphql(validator(email))]
    email: String,
    #[graphql(secret)]
    password: String,
    #[graphql(secret)]
    password_confirmation: String,
    terms: bool,
}

#[derive(InputObject)]
pub struct LoginInput {
    #[graphql(validator(email, max_length = 255))]
    email: String,
    #[graphql(secret)]
    password: String,
}

#[derive(InputObject)]
pub struct UpdateUserInformationInput {
    name: Option<String>,
    #[graphql(validator(email))]
    email: Option<String>,
}

#[derive(InputObject)]
pub struct UpdatePasswordInput {
    #[graphql(secret)]
    current_password: String,
    #[graphql(secret)]
    new_password: String,
    #[graphql(secret)]
    new_password_confirmation: String,
}

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    pub async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<Me> {
        let db = ctx.data::<DatabaseConnection>()?;

        let user = Register::register(
            db,
            CreateUserParameters {
                name: input.name,
                email: input.email,
                password: input.password,
                password_confirmation: input.password_confirmation,
                terms: input.terms,
            },
        )
        .await
        .extend()?;

        let session = ctx.data::<Session>()?;

        session.insert("user_id", user.id).await?;

        Ok(Me::from(&user))
    }

    pub async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<Me> {
        let db = ctx.data::<DatabaseConnection>()?;

        let user: &user::Model = &user::Entity::find_by_email(input.email)
            .one(db)
            .await?
            .ok_or_else(|| Error::new("Invalid credentials"))?
            .into();

        if !verify(input.password, &user.password).map_err(AppError::internal)? {
            return Err(Error::new("Invalid credentials"));
        }

        let session = ctx.data::<Session>()?;

        session.insert("user_id", user.id).await?;

        Ok(user.into())
    }

    pub async fn update_user_information(
        &self,
        ctx: &Context<'_>,
        input: UpdateUserInformationInput,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?
            .user
            .clone();

        let db = ctx.data::<DatabaseConnection>()?;

        let result = UpdateUserInformation::update(
            db,
            user,
            UpdateUserInformationParameters {
                name: input.name,
                email: input.email,
            },
        )
        .await
        .extend()?;

        Ok(result)
    }

    pub async fn update_password(
        &self,
        ctx: &Context<'_>,
        input: UpdatePasswordInput,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?
            .user
            .clone();

        let db = ctx.data::<DatabaseConnection>()?;

        UpdateUserPassword::update(
            db,
            user,
            UpdateUserPasswordParameters {
                current_password: input.current_password,
                new_password: input.new_password,
                new_password_confirmation: input.new_password_confirmation,
            },
        )
        .await
        .extend()?;

        Ok(true)
    }

    pub async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let session = ctx.data::<Session>()?;

        session.delete().await?;

        Ok(true)
    }
}
