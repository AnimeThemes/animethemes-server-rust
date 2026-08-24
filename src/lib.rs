use sea_orm::DatabaseConnection;

use crate::{rules::validation_error::ValidationError, schema::AppSchema};

pub mod actions;
pub mod db;
pub mod entities;
pub mod enums;
pub mod environment;
pub mod graphql;
pub mod middlewares;
pub mod policies;
pub mod rules;
pub mod schema;
pub mod scopes;
pub mod session;
pub mod traits;
pub mod typesense;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub schema: AppSchema,
}

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
        Self::Internal(error.into())
    }
}
