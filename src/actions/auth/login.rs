use sea_orm::DatabaseConnection;

use crate::{
    AppError,
    entities::auth::user::{self},
};

#[derive(Clone)]
pub struct LoginActionParameters {
    pub email: String,
    pub password: String,
}

pub struct LoginAction;

impl LoginAction {
    pub async fn login(
        db: &DatabaseConnection,
        params: LoginActionParameters,
    ) -> Result<user::Model, AppError> {
        let user = user::Entity::find_by_email(params.email)
            .one(db)
            .await?
            .ok_or(AppError::Unauthorized)?;

        if !bcrypt::verify(params.password, &user.password).map_err(AppError::internal)? {
            return Err(AppError::Unauthorized);
        }

        Ok(user)
    }
}
