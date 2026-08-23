use anyhow::{Result, bail};
use bcrypt::{hash, verify};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

use crate::entities::auth::user;

#[derive(Clone)]
pub struct UpdateUserPasswordParameters {
    pub current_password: String,
    pub new_password: String,
}

pub struct UpdateUserPassword;

impl UpdateUserPassword {
    pub async fn validate(user: &user::Model, params: &UpdateUserPasswordParameters) -> Result<()> {
        if !verify(&params.current_password, &user.password)? {
            bail!("Invalid current password");
        }

        Ok(())
    }

    pub async fn update(
        db: &DatabaseConnection,
        user: user::Model,
        params: UpdateUserPasswordParameters,
    ) -> Result<bool> {
        Self::validate(&user, &params).await?;

        let mut user: user::ActiveModel = user.clone().into();

        user.password = Set(hash(&params.new_password, 12)?);

        user.update(db).await?;

        Ok(true)
    }
}
