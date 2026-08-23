use anyhow::{Result, bail};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, SelectExt,
};

use crate::entities::auth::user;

#[derive(Clone)]
pub struct UpdateUserInformationParameters {
    pub name: Option<String>,
    pub email: Option<String>,
}

pub struct UpdateUserInformation;

impl UpdateUserInformation {
    pub async fn validate(
        db: &DatabaseConnection,
        user: &user::Model,
        params: &UpdateUserInformationParameters,
    ) -> Result<()> {
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
            bail!("User already exists with username or email.");
        }

        Ok(())
    }

    pub async fn update(
        db: &DatabaseConnection,
        user: user::Model,
        params: UpdateUserInformationParameters,
    ) -> Result<bool> {
        Self::validate(db, &user, &params).await?;

        let mut user: user::ActiveModel = user.into();

        if let Some(name) = params.name {
            user.name = Set(name);
        }

        if let Some(email) = params.email {
            user.email = Set(email);
            user.email_verified_at = Set(None);
        }

        user.update(db).await?;

        Ok(true)
    }
}
