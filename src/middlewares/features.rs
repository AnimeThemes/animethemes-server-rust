use crate::{entities::admin::feature, features::functions::FeatureManager};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use tower_sessions::Session;

use crate::AppState;

pub async fn features_middleware(
    State(state): State<AppState>,
    _session: Session,
    mut request: Request,
    next: Next,
) -> Response {
    let db = state.db;

    let flags = feature::Entity::find()
        .filter(feature::Column::ScopeType.eq("global"))
        .all(&db)
        .await
        .unwrap_or_default();

    request.extensions_mut().insert(FeatureManager { flags });

    next.run(request).await
}
