use sea_orm::DatabaseConnection;

use crate::schema::AppSchema;

pub mod actions;
pub mod db;
pub mod entities;
pub mod enums;
pub mod environment;
pub mod graphql;
pub mod middlewares;
pub mod policies;
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
