use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::AppError;

pub async fn require_user_agent_middleware(request: Request, next: Next) -> Response {
    let user_agent = request.headers().get("User-Agent");

    match user_agent {
        Some(_agent) => {
            return next.run(request).await;
        }
        None => {
            return AppError::ForbiddenWithMessage("Missing User-Agent header".to_string())
                .into_response();
        }
    }
}
