use animethemes_server_rust::db;
// use crate::entities;
// use crate::enums;
// use crate::scopes;
// use crate::traits;
// use crate::typesense;
// use crate::actions;
// use crate::environment;
// use crate::graphql;
// use crate::middlewares;
// use crate::policies;
use animethemes_server_rust::schema;
// use crate::session;

use animethemes_server_rust::typesense::client::create_typesense_client;
use animethemes_server_rust::typesense::client::init_typesense;
use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    middleware::from_fn_with_state,
    routing::{get, post},
};
use dotenvy::dotenv;
use tower_http::cors::{AllowOrigin, CorsLayer};
use url::Url;

use animethemes_server_rust::{
    AppState,
    middlewares::current_user::current_user_middleware,
    schema::{graphiql, graphql_handler},
    session::create_session_layer,
};

use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = db::connect().await;

    let typesense = create_typesense_client();

    init_typesense(typesense.clone());

    let schema = schema::create_schema(db.clone());

    let state = AppState { db, schema };

    let app_url = env::var("APP_URL").expect("APP_URL must be set in .env");
    let app_port = env::var("APP_PORT").unwrap_or(80.to_string());

    let parsed_url = Url::parse(&app_url).unwrap();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(
            move |origin: &HeaderValue, _request_parts| {
                let Ok(origin) = origin.to_str() else {
                    return false;
                };

                let Ok(origin) = Url::parse(origin) else {
                    return false;
                };

                origin.scheme() == parsed_url.scheme() && origin.host() == parsed_url.host()
            },
        ))
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
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
