use animethemes_server_rust::entities::auth::user;
use async_graphql::{Context, Error, InputObject, Object, Result};
use bcrypt::{hash, verify};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, PaginatorTrait};
use tower_sessions::Session;

use crate::{
    graphql::types::auth::me::Me, middlewares::current_user::CurrentUser, policies::AppError,
};

#[derive(InputObject)]
pub struct SignUpInput {
    name: String,
    #[graphql(validator(email))]
    email: String,
    #[graphql(validator(min_length = 8))]
    password: String,
    #[graphql(validator(min_length = 8))]
    password_confirm: String,
    terms: bool,
}

#[derive(InputObject)]
pub struct LoginInput {
    #[graphql(validator(email))]
    email: String,
    password: String,
}

#[derive(InputObject)]
pub struct UpdateUserInformationInput {
    name: Option<String>,
    #[graphql(validator(min_length = 8))]
    email: Option<String>,
}

#[derive(InputObject)]
pub struct UpdatePasswordInput {
    current_password: String,
    #[graphql(validator(min_length = 8))]
    new_password: String,
    #[graphql(validator(min_length = 8))]
    new_password_confirm: String,
}

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    pub async fn sign_up(&self, ctx: &Context<'_>, input: SignUpInput) -> Result<Me> {
        if !input.terms {
            return Err(Error::new("You must accept the Terms to proceed."));
        }

        if input.password != input.password_confirm {
            return Err(Error::new("The password confirmation does not match."));
        }

        let db = ctx.data::<DatabaseConnection>()?;

        let exists = user::Entity::find_by_email(input.email.clone())
            .count(db)
            .await?;

        if exists > 0 {
            return Err(Error::new("User already exists with this email."));
        }

        let session = ctx.data::<Session>()?;

        let user_active = user::ActiveModel {
            name: Set(input.name),
            email: Set(input.email),
            password: Set(hash(&input.password, 12)?),
            ..Default::default()
        };

        let user: &user::Model = &user_active.insert(db).await?;

        session.insert("user_id", user.id).await?;

        Ok(user.into())
    }

    pub async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<Me> {
        let db = ctx.data::<DatabaseConnection>()?;

        let user: &user::Model = &user::Entity::find_by_email(input.email)
            .one(db)
            .await?
            .ok_or_else(|| Error::new("Invalid credentials"))?
            .into();

        if !verify(input.password, &user.password)? {
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

        let mut user: user::ActiveModel = user.into();

        if let Some(name) = input.name {
            user.name = Set(name);
        }

        if let Some(email) = input.email {
            user.email = Set(email);
            user.email_verified_at = Set(None);
        }

        let db = ctx.data::<DatabaseConnection>()?;

        user.update(db).await?;

        Ok(true)
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

        if input.new_password != input.new_password_confirm {
            return Err(Error::new("The password confirmation does not match"));
        }

        if !verify(&input.current_password, &user.password)? {
            return Err(Error::new("Invalid current password"));
        }

        let password_hash = hash(&input.new_password, 12)?;

        let mut user: user::ActiveModel = user.clone().into();

        user.password = Set(password_hash);

        let db = ctx.data::<DatabaseConnection>()?;

        user.update(db).await?;

        Ok(true)
    }
}
