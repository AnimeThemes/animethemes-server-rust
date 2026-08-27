use std::env;

use axum::{
    extract::{Path, Query},
    response::Redirect,
};
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::Deserialize;

use crate::{
    AppError,
    actions::auth::{roles::RoleAction, verify_email::VerifyEmail},
    entities::auth::{role::Roles, user},
};

#[derive(Debug, Deserialize)]
pub struct VerifyEmailQuery {
    token: String,
    expires: i64,
    signature: String,
}

#[debug_handler(state = AppContext)]
pub async fn verify_email(
    SharedStore(db): SharedStore<DatabaseConnection>,
    Path(user_id): Path<u64>,
    Query(query): Query<VerifyEmailQuery>,
) -> Result<Redirect, AppError> {
    if Utc::now().timestamp() > query.expires {
        return Err(AppError::ForbiddenWithMessage(
            "Verification link has expired".to_string(),
        ));
    }

    let user = user::Entity::find_by_id(user_id)
        .one(&db)
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
        let mut active_user: user::ActiveModel = user.into();

        active_user.email_verified_at = Set(Some(Utc::now()));

        let user = active_user.update(&db).await?;

        RoleAction::assign_role(&db, &user, Roles::Verified).await?;
    }

    let redirect = env::var("CLIENT_PROFILE_URL")
        .unwrap_or_else(|_| "https://animethemes.moe/profile".to_string());

    Ok(Redirect::to(&redirect))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/auth/email")
        .add("/verify/{user_id}", get(verify_email))
}
