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
    let url = env::var("TYPESENSE_URL").expect("TYPESENSE_URL is required");

    let api_key = env::var("TYPESENSE_API_KEY").expect("TYPESENSE_API_KEY is required");

    let client = Client::builder()
        .nodes(vec![url])
        .api_key(api_key)
        .healthcheck_interval(Duration::from_secs(7))
        .retry_policy(ExponentialBackoff::builder().build_with_max_retries(5))
        .build()
        .expect("Error on building Typesense client");

    Arc::new(client)
}

pub fn init_typesense(client: TypesenseClient) -> &'static TypesenseClient {
    TYPESENSE.get_or_init(|| client)
}
