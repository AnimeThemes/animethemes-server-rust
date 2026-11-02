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
    entities::content::audio,
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
        .enabled(Feature::AllowAudioStreams, None)
        .await?;

    let audio = audio::Entity::find()
        .filter(audio::Column::Basename.eq(basename))
        .filter(without_trashed::<audio::Entity>())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let disposition = if query.contains_key("download") {
        Disposition::Attachment
    } else {
        Disposition::Inline
    };

    let nginx_redirect =
        env::var("AUDIO_NGINX_REDIRECT").expect("AUDIO_NGINX_REDIRECT is required");

    let file = Streamable {
        disk: "audios_nyc",
        path: &audio.path,
        basename: &audio.basename,
        mimetype: &audio.mimetype,
        nginx_redirect: &nginx_redirect,
    };

    let response = NginxStream::stream(&ctx, &file, disposition).await;

    response.map_err(|e| AppError::Internal(anyhow::anyhow!("Audio streaming failed: {e}")))
}

pub fn routes() -> Routes {
    let audio_path = env::var("AUDIO_PATH").ok();
    let audio_url = env::var("AUDIO_URL").ok();

    let mut routes = Routes::new();

    if let Some(audio_path) = audio_path {
        routes = routes.prefix(&audio_path);
    }

    if let Some(audio_url) = audio_url {
        routes = routes.layer(middleware::from_fn_with_state(audio_url, media_host_only));
    }

    routes.get("/{basename}", show)
}
