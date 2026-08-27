use crate::{
    entities::{auth::role::Roles, list::playlist},
    enums::list::playlistvisibility::PlaylistVisibility,
};

use crate::{
    middlewares::current_user::CurrentUser,
    policies::{Policy, PolicyAction, PolicyResponse, has_any_role},
};

pub struct PlaylistTrackPolicy;

impl Policy<&playlist::Model> for PlaylistTrackPolicy {
    fn authorize(
        user: Option<&CurrentUser>,
        action: &PolicyAction,
        model: Option<&playlist::Model>,
    ) -> PolicyResponse {
        match action {
            PolicyAction::ViewAny => Self::view_any(user, model.expect("Model expected")),
            PolicyAction::View => Self::view(user, model.expect("Model expected")),
            PolicyAction::Create => Self::create(
                user.expect("Unauthenticated"),
                model.expect("Model expected"),
            ),
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

impl PlaylistTrackPolicy {
    fn view_any(user: Option<&CurrentUser>, playlist: &playlist::Model) -> PolicyResponse {
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

    fn create(user: &CurrentUser, playlist: &playlist::Model) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, &vec![Roles::Verified]);

        if has_role && playlist.user_id == user.user.id {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
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
