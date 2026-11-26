use crate::entities::admin::featuredtheme;
use crate::entities::auth::role::Roles;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::{Policy, PolicyAction, PolicyResponse, has_any_role};

pub struct FeaturedThemePolicy;

impl Policy<&featuredtheme::Model> for FeaturedThemePolicy {
    fn before(user: Option<&CurrentUser>, action: &PolicyAction) -> Option<PolicyResponse> {
        if let Some(before) = <Self as Policy<&featuredtheme::Model>>::global_before(user, action) {
            return Some(before);
        }

        None
    }

    fn authorize(
        user: Option<&CurrentUser>,
        action: &PolicyAction,
        model: Option<&featuredtheme::Model>,
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

impl FeaturedThemePolicy {
    fn view_any() -> PolicyResponse {
        PolicyResponse::Allow
    }

    fn view(_user: Option<&CurrentUser>, featuredtheme: &featuredtheme::Model) -> PolicyResponse {
        if time::OffsetDateTime::now_utc() < featuredtheme.start_at {
            return PolicyResponse::DenyAsNotFound;
        }

        PolicyResponse::Allow
    }

    fn create(user: &CurrentUser) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, vec![Roles::Admin]);

        if has_role {
            return PolicyResponse::Allow;
        }

        PolicyResponse::Deny
    }

    fn update(user: &CurrentUser, _featuredtheme: &featuredtheme::Model) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, vec![Roles::Admin]);

        if has_role {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
    }

    fn delete(user: &CurrentUser, _featuredtheme: &featuredtheme::Model) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, vec![Roles::Admin]);

        if has_role {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
    }
}
