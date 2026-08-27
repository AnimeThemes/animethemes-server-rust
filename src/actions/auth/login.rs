use sea_orm::DatabaseConnection;
use tower_sessions::Session;

use crate::{
    AppError,
    entities::auth::user::{self},
};

#[derive(Clone)]
pub struct LoginActionParameters<'a> {
    pub email: String,
    pub password: String,
    pub session: &'a Session,
}

pub struct LoginAction;

impl LoginAction {
    pub async fn login(
        db: &DatabaseConnection,
        params: LoginActionParameters<'_>,
    ) -> Result<user::Model, AppError> {
        let user = user::Entity::find_by_email(params.email)
            .one(db)
            .await?
            .ok_or(AppError::Unauthorized)?;

        if !bcrypt::verify(params.password, &user.password).map_err(AppError::internal)? {
            return Err(AppError::Unauthorized);
        }

        params
            .session
            .insert("user_id", user.id)
            .await
            .map_err(AppError::internal)?;

        Ok(user)
    }
}
