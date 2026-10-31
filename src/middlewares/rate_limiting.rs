use std::{
    collections::HashSet,
    env,
    net::{IpAddr, SocketAddr},
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::{ConnectInfo, Request, State},
    http::HeaderValue,
    middleware::Next,
    response::{IntoResponse, Response},
};

use redis::{
    Script,
    aio::{ConnectionManager, ConnectionManagerConfig},
};

use crate::{
    AppError, caches::hostname_cache::HostnameCache, entities::auth::role::Roles,
    middlewares::current_user::CurrentUser, policies::has_any_role,
};

// Atomically:
// - verify if the request can pass
// - increment the counter, if allowed
// - define the TTL on the first request
// - return allowed, count and TTL
const RATE_LIMIT_SCRIPT: &str = r#"
local limit = tonumber(ARGV[1])
local window_ms = tonumber(ARGV[2])

local count = tonumber(redis.call('GET', KEYS[1]) or '0')
local allowed = 0

if count == 0 then
    redis.call('SET', KEYS[1], 1, 'PX', window_ms)
    count = 1
    allowed = 1
elseif count < limit then
    count = redis.call('INCR', KEYS[1])
    allowed = 1
end

local ttl = redis.call('PTTL', KEYS[1])

if ttl < 0 then
    redis.call('PEXPIRE', KEYS[1], window_ms)
    ttl = window_ms
end

return { allowed, count, ttl }
"#;

#[derive(Clone)]
pub struct GraphqlRateLimitState {
    redis: ConnectionManager,
    local_ips: HashSet<IpAddr>,
    limit: u32,
}

impl GraphqlRateLimitState {
    pub fn from_env() -> Self {
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL is required");

        let client = redis::Client::open(redis_url).expect("Invalid Redis URL");

        let redis = client
            .get_connection_manager_lazy(ConnectionManagerConfig::new())
            .expect("Cannot configure Redis manager");

        let local_ips = env::var("APP_LOCAL_IPS")
            .unwrap_or_else(|_| "127.0.0.1,::1".to_string())
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<IpAddr>().expect("Invalid APP_LOCAL_IPS"))
            .collect();

        let limit = env::var("RATE_LIMIT")
            .unwrap_or_else(|_| "90".to_string())
            .parse()
            .unwrap();

        Self {
            redis,
            local_ips,
            limit,
        }
    }

    async fn check(&self, key: &str) -> redis::RedisResult<(bool, i64, i64)> {
        let mut conn = self.redis.clone();

        let (allowed, count, ttl_ms): (i64, i64, i64) = Script::new(RATE_LIMIT_SCRIPT)
            .key(key)
            .arg(self.limit)
            .arg(60000)
            .invoke_async(&mut conn)
            .await?;

        Ok((allowed == 1, count, ttl_ms))
    }
}

fn add_rate_limit_headers(
    response: &mut Response,
    count: i64,
    ttl_ms: i64,
    limit: u32,
    retry_after: Option<u64>,
) {
    let remaining = (limit as i64 - count).max(0);

    let reset_after = ((ttl_ms.max(0) as u64) + 999) / 1000;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let headers = response.headers_mut();

    if let Some(retry_after) = retry_after {
        headers.insert(
            "Retry-After",
            HeaderValue::from_str(&retry_after.to_string()).unwrap(),
        );
    }

    headers.insert(
        "X-RateLimit-Limit",
        HeaderValue::from_str(limit.to_string().as_str()).unwrap(),
    );
    headers.insert(
        "X-RateLimit-Remaining",
        HeaderValue::from_str(&remaining.to_string()).unwrap(),
    );
    headers.insert(
        "X-RateLimit-Reset",
        HeaderValue::from_str(&(now + reset_after).to_string()).unwrap(),
    );
}

pub async fn rate_limiting_middleware(
    State(state): State<GraphqlRateLimitState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let current_user = request.extensions().get::<CurrentUser>();

    // Bypass for admins
    if let Some(current_user) = current_user {
        if has_any_role(&current_user.roles, vec![Roles::Admin]) {
            return Ok(next.run(request).await);
        }
    }

    // Get the client IP
    let Some(ConnectInfo(peer)) = request.extensions().get::<ConnectInfo<SocketAddr>>() else {
        return Err(AppError::Internal(anyhow::Error::msg(
            "Couldn't get client IP",
        )));
    };

    let peer_ip = peer.ip();

    let hostname_cache = request
        .extensions()
        .get::<HostnameCache>()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Hostname cache not initialized")))?;

    // Only accept forwarded IP if the connection came from a trusted local IP
    let ip = if hostname_cache.is_allowed(peer_ip) || state.local_ips.contains(&peer_ip) {
        let forwarded_ip = request.headers().get("x-forwarded-ip");

        match forwarded_ip {
            // Without forwarded IP, allow bypass
            None => {
                return Ok(next.run(request).await);
            }
            // Forwarded IP is valid
            Some(value) => {
                let parsed = value
                    .to_str()
                    .ok()
                    .and_then(|v| v.trim().parse::<IpAddr>().ok());

                let Some(ip) = parsed else {
                    return Err(AppError::Internal(anyhow::Error::msg(
                        "Invalid forwarded IP",
                    )));
                };

                ip
            }
        }
    } else {
        peer_ip
    };

    // Get the key for rate limiting
    let key = match current_user {
        Some(current_user) => {
            format!("rate_limiting:graphql:user:{}", current_user.user.id)
        }
        None => {
            format!("rate_limiting:graphql:ip:{ip}")
        }
    };

    // Execute the atomic check in Redis
    let (allowed, count, ttl_ms) = match state.check(&key).await {
        Ok(result) => result,
        Err(error) => {
            tracing::error!(%error, "GraphQL rate limiter Redis error");

            return Err(AppError::internal(error));
        }
    };

    // Rate limit exceeded
    if !allowed {
        let mut response = AppError::RateLimitingExceeded.into_response();

        add_rate_limit_headers(
            &mut response,
            count,
            ttl_ms,
            state.limit,
            Some(((ttl_ms.max(0) as u64) + 999) / 1000),
        );

        return Ok(response);
    }

    // Execute the request normally
    let mut response = next.run(request).await;

    add_rate_limit_headers(&mut response, count, ttl_ms, state.limit, None);

    Ok(response)
}
