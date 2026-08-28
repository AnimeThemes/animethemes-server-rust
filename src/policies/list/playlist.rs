use crate::{
    entities::{
        auth::{role::Roles, sanction::Sanctions},
        list::playlist,
    },
    enums::list::playlistvisibility::PlaylistVisibility,
    policies::get_sanction,
};

use crate::{
    middlewares::current_user::CurrentUser,
    policies::{Policy, PolicyAction, PolicyResponse, has_any_role},
};

pub struct PlaylistPolicy;

impl Policy<&playlist::Model> for PlaylistPolicy {
    fn before(user: Option<&CurrentUser>, action: &PolicyAction) -> Option<PolicyResponse> {
        if let Some(before) = <Self as Policy<&playlist::Model>>::global_before(user, action) {
            return Some(before);
        }

        match user {
            Some(user) => match action {
                PolicyAction::Create | PolicyAction::Update | PolicyAction::Delete => {
                    if let Some((user_sanction, _)) =
                        get_sanction(&user.sanctions, vec![Sanctions::PlaylistManagement])
                    {
                        let message = Sanctions::PlaylistManagement
                            .get_forbidden_message(user_sanction.get_forbidden_message());

                        return Some(PolicyResponse::DenyWithMessage(message.to_string()));
                    }
                }
                _ => {}
            },
            None => {}
        }

        None
    }

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
        let has_role = has_any_role(&user.roles, vec![Roles::Verified]);

        if has_role {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyWithMessage("Please verify your email to create a playlist".to_string())
    }

    fn update(user: &CurrentUser, playlist: &playlist::Model) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, vec![Roles::Verified]);

        if has_role && playlist.user_id == user.user.id {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
    }

    fn delete(user: &CurrentUser, playlist: &playlist::Model) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, vec![Roles::Verified]);

        if has_role && playlist.user_id == user.user.id {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
    }
}
