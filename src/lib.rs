use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::rules::validation_error::ValidationError;

pub mod actions;
pub mod app;
pub mod controllers;
pub mod data;
pub mod db;
pub mod entities;
pub mod enums;
pub mod environment;
pub mod features;
pub mod graphql;
pub mod initializers;
pub mod mail;
pub mod middlewares;
pub mod policies;
pub mod rules;
pub mod schema;
pub mod scopes;
pub mod session;
pub mod tasks;
pub mod traits;
pub mod typesense;
pub mod views;
pub mod workers;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Unauthenticated")]
    Unauthenticated,
    #[error("Validation")]
    Validation(Vec<ValidationError>),
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("{0}")]
    ForbiddenWithMessage(String),
    #[error("Internal server error")]
    Database(#[from] sea_orm::DbErr),
    #[error("Internal server error")]
    Internal(#[source] anyhow::Error),
}

impl AppError {
    pub fn internal<E>(error: E) -> Self
    where
        E: Into<anyhow::Error>,
    {
        let error = error.into();

        tracing::error!(
            error = %format!("{error:#}"),
            "internal server error"
        );

        Self::Internal(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Unauthenticated => {
                (StatusCode::UNAUTHORIZED, "Unauthenticated").into_response()
            }
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden").into_response(),
            AppError::ForbiddenWithMessage(message) => {
                (StatusCode::FORBIDDEN, message).into_response()
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            AppError::Validation(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Validation error").into_response()
            }
            AppError::Database(error) => {
                tracing::error!(error = ?error, "database error");

                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
            AppError::Internal(error) => {
                tracing::error!(
                    error = %format!("{error:#}"),
                    "internal error"
                );

                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}
