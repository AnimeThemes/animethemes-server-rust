use std::env;

use chrono::{Duration, Utc};
use hmac::{Hmac, KeyInit, Mac};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    AppError,
    entities::auth::user::{self},
    mail::{mailer::Mail, messages::Messages},
};

pub struct VerifyEmail;

impl VerifyEmail {
    pub async fn send_verification_email(user: &user::Model) -> Result<(), AppError> {
        if user.email_verified_at.is_some() {
            return Err(AppError::ForbiddenWithMessage(
                "Email already verified".to_string(),
            ));
        }

        let message = Messages::verification_email(
            &user.name,
            &user.email,
            &Self::verification_url(user.id, &user.email)?,
        );

        Mail::send(message).await?;

        Ok(())
    }

    fn verification_url(user_id: u64, email: &str) -> Result<String, AppError> {
        let app_url = env::var("APP_URL").expect("APP_URL must be set in .env");
        let app_key = env::var("APP_KEY").expect("APP_KEY must be set in .env");

        let email_hash = hex::encode(Sha256::digest(email.as_bytes()));
        let token = Uuid::new_v4().simple().to_string();
        let expires = (Utc::now() + Duration::minutes(60)).timestamp();

        let payload = format!("email-verification|{user_id}|{email_hash}|{token}|{expires}");

        let key = hex::decode(app_key).map_err(AppError::internal)?;

        let mut mac = Hmac::<Sha256>::new_from_slice(&key).map_err(AppError::internal)?;

        mac.update(payload.as_bytes());

        let signature = hex::encode(mac.finalize().into_bytes());

        Ok(format!(
            "{}/api/auth/email/verify/{}?token={}&expires={}&signature={}",
            app_url.trim_end_matches('/'),
            user_id,
            token,
            expires,
            signature,
        ))
    }

    pub fn verify_email_signature(
        user_id: u64,
        email: &str,
        token: &str,
        expires: i64,
        signature: &str,
    ) -> Result<(), AppError> {
        let app_key = env::var("APP_KEY").expect("APP_KEY must be set");

        let key = hex::decode(app_key).map_err(AppError::internal)?;

        let email_hash = hex::encode(Sha256::digest(email.as_bytes()));

        let payload = format!("email-verification|{user_id}|{email_hash}|{token}|{expires}");

        let signature = hex::decode(signature).map_err(|_| AppError::Forbidden)?;

        let mut mac = Hmac::<Sha256>::new_from_slice(&key).map_err(AppError::internal)?;

        mac.update(payload.as_bytes());

        mac.verify_slice(&signature)
            .map_err(|_| AppError::Forbidden)?;

        Ok(())
    }
}
