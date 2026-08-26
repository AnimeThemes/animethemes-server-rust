use std::env;

use axum::{
    extract::{Path, Query, State},
    response::Redirect,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;

use crate::{AppError, AppState, actions::auth::verify_email::VerifyEmail, entities::auth::user};

#[derive(Deserialize)]
pub struct VerifyEmailQuery {
    token: String,
    expires: i64,
    signature: String,
}

pub async fn verify_email(
    State(state): State<AppState>,
    Path(user_id): Path<u64>,
    Query(query): Query<VerifyEmailQuery>,
) -> Result<Redirect, AppError> {
    if Utc::now().timestamp() > query.expires {
        return Err(AppError::ForbiddenWithMessage(
            "Verification link has expired".to_string(),
        ));
    }

    let user = user::Entity::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or(AppError::Forbidden)?;

    VerifyEmail::verify_email_signature(
        user.id,
        &user.email,
        &query.token,
        query.expires,
        &query.signature,
    )?;

    if user.email_verified_at.is_none() {
        let mut user: user::ActiveModel = user.into();

        user.email_verified_at = Set(Some(Utc::now()));

        user.update(&state.db).await?;
    }

    let redirect =
        env::var("CLIENT_PROFILE_URL").unwrap_or("https://animethemes.moe/profile".to_string());

    Ok(Redirect::to(redirect.as_str()))
}
