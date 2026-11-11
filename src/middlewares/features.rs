use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use loco_rs::app::AppContext;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tower_sessions::Session;

use crate::entities::admin::feature;
use crate::features::functions::FeatureManager;

pub async fn features_middleware(
    State(ctx): State<AppContext>,
    _session: Session,
    mut request: Request,
    next: Next,
) -> Response {
    let flags = feature::Entity::find()
        .filter(feature::Column::ScopeType.eq("global"))
        .all(&ctx.db)
        .await
        .unwrap_or_default();

    request.extensions_mut().insert(FeatureManager { flags });

    next.run(request).await
}
