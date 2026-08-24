use std::{
    env,
    sync::{Arc, OnceLock},
    time::Duration,
};

use typesense::{Client, ExponentialBackoff};

pub type TypesenseClient = Arc<Client>;

static TYPESENSE: OnceLock<TypesenseClient> = OnceLock::new();

pub fn typesense() -> &'static TypesenseClient {
    TYPESENSE
        .get()
        .expect("Typesense client was not initialized")
}

pub fn create_typesense_client() -> TypesenseClient {
    let host = env::var("TYPESENSE_HOST").expect("TYPESENSE_HOST is required");

    let port = env::var("TYPESENSE_PORT").expect("TYPESENSE_PORT is required");

    let protocol = env::var("TYPESENSE_PROTOCOL").unwrap_or_else(|_| "http".to_string());

    let api_key = env::var("TYPESENSE_API_KEY").expect("TYPESENSE_API_KEY is required");

    let node_url = format!("{protocol}://{host}:{port}");

    let client = Client::builder()
        .nodes(vec![node_url])
        .api_key(api_key)
        .healthcheck_interval(Duration::from_secs(7))
        .retry_policy(ExponentialBackoff::builder().build_with_max_retries(5))
        .build()
        .expect("Error on building Typesense client");

    Arc::new(client)
}

pub fn init_typesense(client: TypesenseClient) {
    TYPESENSE
        .set(client)
        .expect("Typesense client was already initialized");
}
