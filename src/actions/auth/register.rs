use anyhow::{Result, bail};
use bcrypt::hash;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, SelectExt,
};

use crate::entities::auth::user;

#[derive(Clone)]
pub struct CreateUserParameters {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct Register;

impl Register {
    pub async fn validate(db: &DatabaseConnection, params: &CreateUserParameters) -> Result<()> {
        let password = &params.password;

        if password.chars().count() < 8 {
            bail!("The password must be at least 8 characters.");
        }

        if !password.chars().any(|c| c.is_uppercase()) {
            bail!("The password must contain at least one uppercase letter.");
        }

        if !password.chars().any(|c| c.is_lowercase()) {
            bail!("The password must contain at least one lowercase letter.");
        }

        if !password.chars().any(|c| c.is_alphabetic()) {
            bail!("The password must contain at least one letter.");
        }

        if !password.chars().any(|c| c.is_numeric()) {
            bail!("The password must contain at least one number.");
        }

        if !password.chars().any(|c| c.is_ascii_punctuation()) {
            bail!("The password must contain at least one symbol.");
        }

        let exists = user::Entity::find()
            .filter(
                Condition::any()
                    .add(user::Column::Name.eq(params.name.clone()))
                    .add(user::Column::Email.eq(params.email.clone())),
            )
            .exists(db)
            .await?;

        if exists {
            bail!("User already exists with username or email.");
        }

        Ok(())
    }

    pub async fn register(
        db: &DatabaseConnection,
        params: CreateUserParameters,
    ) -> Result<user::Model> {
        Self::validate(db, &params).await?;

        let model = user::ActiveModel {
            name: Set(params.name.clone()),
            email: Set(params.email.clone()),
            password: Set(hash(params.password, 12)?),
            ..Default::default()
        };

        let user = model.insert(db).await?;

        Ok(user)
    }
}
