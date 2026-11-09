use crate::{
    entities::{
        auth::{role::Roles, sanction::Sanctions},
        user::favorite,
    },
    policies::get_sanction,
};

use crate::{
    middlewares::current_user::CurrentUser,
    policies::{Policy, PolicyAction, PolicyResponse, has_any_role},
};

pub struct FavoritePolicy;

impl Policy<&favorite::Model> for FavoritePolicy {
    fn before(user: Option<&CurrentUser>, action: &PolicyAction) -> Option<PolicyResponse> {
        if let Some(before) = <Self as Policy<&favorite::Model>>::global_before(user, action) {
            return Some(before);
        }

        match user {
            Some(user) => match action {
                PolicyAction::Create | PolicyAction::Update | PolicyAction::Delete => {
                    if let Some((user_sanction, _)) =
                        get_sanction(&user.sanctions, vec![Sanctions::FavoriteManagement])
                    {
                        let message = Sanctions::FavoriteManagement
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
        model: Option<&favorite::Model>,
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

impl FavoritePolicy {
    fn view_any() -> PolicyResponse {
        PolicyResponse::Allow
    }

    fn view(user: Option<&CurrentUser>, favorite: &favorite::Model) -> PolicyResponse {
        if let Some(user) = user
            && favorite.user_id == user.user.id
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

        PolicyResponse::DenyWithMessage(
            "Please verify your email to favorite an entity".to_string(),
        )
    }

    fn update(user: &CurrentUser, favorite: &favorite::Model) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, vec![Roles::Verified]);

        if has_role && favorite.user_id == user.user.id {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
    }

    fn delete(user: &CurrentUser, favorite: &favorite::Model) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, vec![Roles::Verified]);

        if has_role && favorite.user_id == user.user.id {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
    }
}
