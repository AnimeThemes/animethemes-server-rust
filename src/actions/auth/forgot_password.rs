use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use sha2::{Digest, Sha256};

use crate::{
    AppError,
    entities::auth::{
        password_reset_tokens,
        user::{self},
    },
    mail::{mailer::Mail, messages::Messages},
};

pub struct ForgotPassword;

impl ForgotPassword {
    fn hash_token(token: &str) -> String {
        hex::encode(Sha256::digest(token.as_bytes()))
    }

    pub async fn send_reset_password_email(
        db: &DatabaseConnection,
        email: String,
    ) -> Result<(), AppError> {
        let user = user::Entity::find_by_email(&email).one(db).await?;

        if let Some(user) = user {
            password_reset_tokens::Entity::delete_many()
                .filter(password_reset_tokens::Column::Email.eq(&user.email))
                .exec(db)
                .await?;

            let token = uuid::Uuid::new_v4().to_string();

            password_reset_tokens::ActiveModel {
                email: Set(user.email.clone()),
                token: Set(Self::hash_token(&token)),
                ..Default::default()
            }
            .insert(db)
            .await?;

            let message = Messages::reset_password_email(&user.name, &user.email, &token);

            Mail::send(message).await?;
        }

        Ok(())
    }
}
