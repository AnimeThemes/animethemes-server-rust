use crate::{
    entities::{auth::role::Roles, list::playlist},
    enums::list::playlistvisibility::PlaylistVisibility,
};

use crate::{
    middlewares::current_user::CurrentUser,
    policies::{Policy, PolicyAction, PolicyResponse, has_any_role},
};

pub struct PlaylistPolicy;

impl Policy<&playlist::Model> for PlaylistPolicy {
    fn authorize(
        user: Option<&CurrentUser>,
        action: &PolicyAction,
        model: Option<&playlist::Model>,
    ) -> PolicyResponse {
        match action {
            PolicyAction::ViewAny => Self::view_any(),
            PolicyAction::View => Self::view(user, model.expect("Model expected")),
            PolicyAction::Create => Self::create(user.expect("Unauthenticated")),
            PolicyAction::Update => Self::update(
                user.expect("Unauthenticated"),
                model.expect("Model expected"),
            ),
            PolicyAction::Delete => Self::delete(
                user.expect("Unauthenticated"),
                model.expect("Model expected"),
            ),
        }
    }
}

impl PlaylistPolicy {
    fn view_any() -> PolicyResponse {
        PolicyResponse::Allow
    }

    fn view(user: Option<&CurrentUser>, playlist: &playlist::Model) -> PolicyResponse {
        if playlist.visibility == PlaylistVisibility::Public {
            return PolicyResponse::Allow;
        }

        if let Some(user) = user
            && playlist.user_id == user.user.id
        {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
    }

    fn create(user: &CurrentUser) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, &vec![Roles::Verified]);

        if has_role {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyWithMessage("Please verify your email to create a playlist".to_string())
    }

    fn update(user: &CurrentUser, playlist: &playlist::Model) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, &vec![Roles::Verified]);

        if has_role && playlist.user_id == user.user.id {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
    }

    fn delete(user: &CurrentUser, playlist: &playlist::Model) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, &vec![Roles::Verified]);

        if has_role && playlist.user_id == user.user.id {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
    }
}
