use std::{collections::HashMap, env};

use axum::{
    Extension,
    extract::{Path, Query, State},
    middleware::{self},
    response::Response,
};
use loco_rs::{app::AppContext, prelude::Routes};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    AppError,
    actions::stream::nginx_stream::{Disposition, NginxStream, Streamable},
    controllers::streams::media_host_only,
    entities::content::video,
    enums::features::Feature,
    features::functions::FeatureManager,
    scopes::without_trashed,
};

async fn show(
    State(ctx): State<AppContext>,
    Extension(feature_manager): Extension<FeatureManager>,
    Path(basename): Path<String>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    feature_manager
        .enabled(Feature::AllowVideoStreams, None)
        .await?;

    let video = video::Entity::find()
        .filter(video::Column::Basename.eq(basename))
        .filter(without_trashed::<video::Entity>())
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
