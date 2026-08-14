use animethemes_server_rust::entities::auth::{role, user};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use sea_orm::EntityTrait;

use tower_sessions::Session;

use crate::AppState;

#[derive(Clone)]
pub struct CurrentUser {
    pub user: user::Model,
    pub roles: Vec<role::Model>,
}

pub async fn current_user_middleware(
    State(state): State<AppState>,
    session: Session,
    mut request: Request,
    next: Next,
) -> Response {
    let db = state.db;

    let Some(user_id) = session.get::<u64>("user_id").await.ok().flatten() else {
        return next.run(request).await;
    };

    let users = match user::Entity::find_by_id(user_id)
        .find_with_related(role::Entity)
        .all(&db)
        .await
    {
        Ok(users) => users,
        Err(error) => {
            println!("Failed to load current user {user_id}: {error}");
            return next.run(request).await;
        }
    };

    let Some((user, roles)) = users.into_iter().next() else {
        println!("User {user_id} from session was not found");
        return next.run(request).await;
    };

    request.extensions_mut().insert(CurrentUser {
        user: user,
        roles: roles,
    });

    next.run(request).await
}
