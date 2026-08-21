use crate::entities::auth::user;
use async_graphql::{Context, Error, InputObject, Object, Result};
use bcrypt::{hash, verify};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, SelectExt};
use tower_sessions::Session;

use crate::{
    graphql::types::auth::me::Me, middlewares::current_user::CurrentUser, policies::AppError,
};

#[derive(InputObject)]
pub struct RegisterInput {
    #[graphql(validator(min_length = 1, max_length = 35))]
    name: String,
    #[graphql(validator(email, max_length = 255))]
    email: String,
    #[graphql(validator(min_length = 8), secret)]
    password: String,
    #[graphql(validator(min_length = 8), secret)]
    password_confirm: String,
    terms: bool,
}

impl RegisterInput {
    pub fn validate(&self) -> Result<()> {
        if !self.terms {
            return Err(Error::new("You must accept the Terms to proceed."));
        }

        if self.password != self.password_confirm {
            return Err(Error::new("The password confirmation does not match."));
        }

        Ok(())
    }
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
    #[graphql(validator(min_length = 1, max_length = 35))]
    name: Option<String>,
    #[graphql(validator(email, max_length = 255))]
    email: Option<String>,
}

#[derive(InputObject)]
pub struct UpdatePasswordInput {
    #[graphql(secret)]
    current_password: String,
    #[graphql(validator(min_length = 8), secret)]
    new_password: String,
    #[graphql(validator(min_length = 8), secret)]
    new_password_confirm: String,
}

impl UpdatePasswordInput {
    pub fn validate(&self) -> Result<()> {
        if self.new_password != self.new_password_confirm {
            return Err(Error::new("The password confirmation does not match."));
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    pub async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<Me> {
        input.validate()?;

        let db = ctx.data::<DatabaseConnection>()?;

        let exists = user::Entity::find_by_email(input.email.clone())
            .exists(db)
            .await?;

        if exists {
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
        input.validate()?;

        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?
            .user
            .clone();

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

    pub async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let session = ctx.data::<Session>()?;

        session.delete().await?;

        Ok(true)
    }
}
