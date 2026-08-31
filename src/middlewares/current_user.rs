use crate::{
    entities::auth::{role, sanction, user, user_sanctions},
    scopes::auth::user_sanctions::current_sanctions,
};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use loco_rs::app::AppContext;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use tower_sessions::Session;

#[derive(Clone)]
pub struct CurrentUser {
    pub user: user::Model,
    pub roles: Vec<role::Model>,
    pub sanctions: Vec<(user_sanctions::Model, sanction::Model)>,
}

pub async fn current_user_middleware(
    State(ctx): State<AppContext>,
    session: Session,
    mut request: Request,
    next: Next,
) -> Response {
    let db = ctx.db;

    let Some(user_id) = session.get::<u64>("user_id").await.ok().flatten() else {
        return next.run(request).await;
    };

    let users = match user::Entity::find_by_id(user_id)
        .find_with_related(role::Entity)
        .all(&db)
        .await
    {
        Ok(users) => users,
        Err(_) => {
            return next.run(request).await;
        }
    };

    let sanctions = match user_sanctions::Entity::find()
        .filter(user_sanctions::Column::UserId.eq(user_id))
        .filter(current_sanctions())
        .find_both_related(sanction::Entity)
        .all(&db)
        .await
    {
        Ok(sanctions) => sanctions,
        Err(_) => Vec::new(),
    };

    let Some((user, roles)) = users.into_iter().next() else {
        return next.run(request).await;
    };

    request.extensions_mut().insert(CurrentUser {
        user: user,
        roles: roles,
        sanctions: sanctions,
    });

    next.run(request).await
}
