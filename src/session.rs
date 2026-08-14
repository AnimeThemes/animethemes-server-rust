use std::env;

use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tower_sessions_redis_store::{
    RedisStore,
    fred::{clients::Pool, interfaces::ClientLike, types::config::Config},
};

use crate::environment::{Environment, get_environment};

pub async fn create_session_layer() -> SessionManagerLayer<RedisStore<Pool>> {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let config = Config::from_url(&redis_url).expect("Invalid REDIS_URL");

    let pool = Pool::new(config, None, None, None, 6).expect("Failed to create Redis pool");

    pool.connect();

    pool.wait_for_connect()
        .await
        .expect("Failed to connect to Redis");

    let store = RedisStore::new(pool);

    let is_production_or_stagging = matches!(
        get_environment(),
        Environment::Production | Environment::Stagging
    );

    SessionManagerLayer::new(store)
        .with_secure(is_production_or_stagging)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::days(30)))
}
