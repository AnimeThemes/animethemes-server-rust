use std::env;

use axum::extract::{Path, Query};
use axum::response::Redirect;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;

use crate::actions::auth::verify_email::VerifyEmail;
use crate::entities::auth::user;
use crate::events::auth::user_verified::UserVerified;
use crate::{AppError, events};

#[derive(Debug, Deserialize)]
pub struct VerifyEmailQuery {
    token: String,
    expires: i64,
    signature: String,
}

#[debug_handler(state = AppContext)]
pub async fn verify_email(
    State(ctx): State<AppContext>,
    Path(user_id): Path<u64>,
    Query(query): Query<VerifyEmailQuery>,
) -> Result<Redirect, AppError> {
    if time::OffsetDateTime::now_utc().unix_timestamp() > query.expires {
        return Err(AppError::ForbiddenWithMessage(
            "Verification link has expired".to_string(),
        ));
    }

    let user = user::Entity::find_by_id(user_id)
        .one(&ctx.db)
        .await?
        .ok_or(AppError::Forbidden)?;

    VerifyEmail::verify_email_signature(
        user.id,
        &user.email,
        &query.token,
        query.expires,
        &query.signature,
    )?;

    let verified_user = if user.email_verified_at.is_none() {
        let mut active_user: user::ActiveModel = user.into();

        active_user.email_verified_at = Set(Some(time::OffsetDateTime::now_utc()));

        active_user.update(&ctx.db).await?
    } else {
        user.clone()
    };

    events::dispatch(
        &ctx,
        UserVerified {
            user: verified_user,
        },
    )
    .await;

    let redirect = env::var("CLIENT_PROFILE_URL")
        .unwrap_or_else(|_| "https://animethemes.moe/profile".to_string());

    Ok(Redirect::to(&redirect))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/auth/email")
        .add("/verify/{user_id}", get(verify_email))
}
