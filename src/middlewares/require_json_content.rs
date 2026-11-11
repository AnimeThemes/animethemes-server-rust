use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use reqwest::StatusCode;
use reqwest::header::CONTENT_TYPE;

pub async fn require_json_content_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let is_json = request
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .is_some_and(|value| value.trim().eq_ignore_ascii_case("application/json"));

    if !is_json {
        return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    Ok(next.run(request).await)
}
