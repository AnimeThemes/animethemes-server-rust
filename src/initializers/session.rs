use std::env;

use loco_rs::app::AppContext;
use loco_rs::environment::Environment;
use tower_sessions::cookie::SameSite;
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::RedisStore;
use tower_sessions_redis_store::fred::clients::Pool;
use tower_sessions_redis_store::fred::interfaces::ClientLike;
use tower_sessions_redis_store::fred::types::config::Config;

pub async fn create_session_layer(ctx: &AppContext) -> SessionManagerLayer<RedisStore<Pool>> {
    let domain = env::var("SESSION_DOMAIN").expect("SESSION_DOMAIN is required in .env");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let config = Config::from_url(&redis_url).expect("Invalid REDIS_URL");

    let pool = Pool::new(config, None, None, None, 6).expect("Failed to create Redis pool");

    pool.connect();

    pool.wait_for_connect()
        .await
        .expect("Failed to connect to Redis");

    let store = RedisStore::new(pool);

    let is_production = matches!(ctx.environment, Environment::Production);

    SessionManagerLayer::new(store)
        .with_secure(is_production)
        .with_domain(domain)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::days(30)))
}
