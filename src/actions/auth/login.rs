use async_graphql::InputObject;
use loco_rs::app::AppContext;

use crate::AppError;
use crate::entities::auth::user::{self};

#[derive(InputObject, Clone)]
#[graphql(name = "LoginInput")]
pub struct LoginActionParameters {
    pub email: String,
    #[graphql(secret)]
    pub password: String,
}

pub struct LoginAction;

impl LoginAction {
    pub async fn login(
        ctx: &AppContext,
        params: LoginActionParameters,
    ) -> Result<user::Model, AppError> {
        let user = user::Entity::find_by_email(params.email)
            .one(&ctx.db)
            .await?
            .ok_or(AppError::Unauthorized)?;

        if !bcrypt::verify(params.password, &user.password).map_err(AppError::internal)? {
            return Err(AppError::Unauthorized);
        }

        Ok(user)
    }
}
