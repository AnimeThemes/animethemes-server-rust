mod actions;
mod environment;
mod graphql;
mod middlewares;
mod policies;
mod schema;
mod session;

use animethemes_server_rust::{db, entities, enums, scopes, typesense};

use axum::{
    Router,
    http::{HeaderValue, Method},
    middleware::from_fn_with_state,
    routing::{get, post},
};
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use tower_http::cors::CorsLayer;

use crate::{
    middlewares::current_user::current_user_middleware,
    schema::{AppSchema, graphiql, graphql_handler},
    session::create_session_layer,
};

use std::env;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub schema: AppSchema,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = db::connect().await;

    let schema = schema::create_schema(db.clone());

    let state = AppState { db, schema };

    let app_url = env::var("APP_URL").expect("APP_URL must be set in .env");
    let app_port = env::var("APP_PORT").unwrap_or(80.to_string());

    let cors = CorsLayer::new()
        .allow_origin(app_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true);

    let session_layer = create_session_layer().await;

    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql_handler))
        .layer(from_fn_with_state(state.clone(), current_user_middleware))
        .layer(session_layer)
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", app_port))
        .await
        .unwrap();

    let url = if app_port == 80.to_string() {
        app_url
    } else {
        format!("{}:{}", app_url, app_port)
    };

    println!("GraphiQL: {}", url);

    axum::serve(listener, app).await.unwrap();
}
