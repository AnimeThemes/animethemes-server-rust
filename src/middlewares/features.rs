use crate::{entities::admin::feature, features::functions::FeatureManager};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use loco_rs::app::AppContext;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use tower_sessions::Session;

pub async fn features_middleware(
    State(ctx): State<AppContext>,
    _session: Session,
    mut request: Request,
    next: Next,
) -> Response {
    let db = ctx
        .shared_store
        .get::<DatabaseConnection>()
        .expect("Database not initialized");

    let flags = feature::Entity::find()
        .filter(feature::Column::ScopeType.eq("global"))
        .all(&db)
        .await
        .unwrap_or_default();

    request.extensions_mut().insert(FeatureManager { flags });

    next.run(request).await
}
