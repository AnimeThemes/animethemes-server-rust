use std::{collections::HashMap, env};

use axum::{
    extract::{Path, Query, Request, State},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use loco_rs::{app::AppContext, prelude::Routes};
use reqwest::{StatusCode, header::HOST};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    AppError,
    actions::stream::nginx_stream::{Disposition, NginxStream, Streamable},
    entities::content::video,
};

async fn show(
    State(ctx): State<AppContext>,
    Path(basename): Path<String>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    let video = video::Entity::find()
        .filter(video::Column::Basename.eq(basename))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let disposition = if query.contains_key("download") {
        Disposition::Attachment
    } else {
        Disposition::Inline
    };

    let nginx_redirect =
        env::var("VIDEO_NGINX_REDIRECT").expect("VIDEO_NGINX_REDIRECT is required");

    let file = Streamable {
        disk: "videos_nyc",
        path: &video.path,
        basename: &video.basename,
        mimetype: &video.mimetype,
        nginx_redirect: &nginx_redirect,
    };

    let response = NginxStream::stream(&ctx, &file, disposition).await;

    response.map_err(|e| AppError::Internal(anyhow::anyhow!("Video streaming failed: {e}")))
}

async fn media_host_only(
    State(video_url): State<String>,
    request: Request,
    next: Next,
) -> Response {
    let allowed = request
        .headers()
        .get(HOST)
        .and_then(|host| host.to_str().ok())
        .is_some_and(|host| host.eq_ignore_ascii_case(video_url.as_str()));

    if !allowed {
        return StatusCode::NOT_FOUND.into_response();
    }

    next.run(request).await
}

pub fn routes() -> Routes {
    let video_path = env::var("VIDEO_PATH").ok();
    let video_url = env::var("VIDEO_URL").ok();

    let mut routes = Routes::new();

    if let Some(video_path) = video_path {
        routes = routes.prefix(&video_path);
    }

    if let Some(video_url) = video_url {
        routes = routes.layer(middleware::from_fn_with_state(video_url, media_host_only));
    }

    routes.get("/{basename}", show)
}
