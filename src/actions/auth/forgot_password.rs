use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use sha2::{Digest, Sha256};

use crate::AppError;
use crate::entities::auth::password_reset_tokens;
use crate::entities::auth::user::{self};
use crate::mail::mailer::Mail;
use crate::mail::messages::Messages;

pub struct ForgotPassword;

impl ForgotPassword {
    fn hash_token(token: &str) -> String {
        hex::encode(Sha256::digest(token.as_bytes()))
    }

    pub async fn send_reset_password_email(
        ctx: &AppContext,
        email: String,
    ) -> Result<(), AppError> {
        let user = user::Entity::find_by_email(&email).one(&ctx.db).await?;

        if let Some(user) = user {
            password_reset_tokens::Entity::delete_many()
                .filter(password_reset_tokens::Column::Email.eq(&user.email))
                .exec(&ctx.db)
                .await?;

            let token = uuid::Uuid::new_v4().to_string();

            password_reset_tokens::ActiveModel {
                email: Set(user.email.clone()),
                token: Set(Self::hash_token(&token)),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await?;

            let message = Messages::reset_password_email(&user.name, &user.email, &token);

            Mail::send(message).await?;
        }

        Ok(())
    }
}
