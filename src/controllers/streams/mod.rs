use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use reqwest::header::HOST;

pub mod audio_stream_controller;
pub mod video_stream_controller;

pub async fn media_host_only(State(url): State<String>, request: Request, next: Next) -> Response {
    let allowed = request
        .headers()
        .get(HOST)
        .and_then(|host| host.to_str().ok())
        .is_some_and(|host| host.eq_ignore_ascii_case(url.as_str()));

    if !allowed {
        return StatusCode::NOT_FOUND.into_response();
    }

    next.run(request).await
}
