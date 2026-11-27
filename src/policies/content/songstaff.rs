use crate::entities::auth::role::Roles;
use crate::entities::content::song_staff;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::{Policy, PolicyAction, PolicyResponse, has_any_role};

pub struct SongStaffPolicy;

impl Policy<&song_staff::Model> for SongStaffPolicy {
    fn before(user: Option<&CurrentUser>, action: &PolicyAction) -> Option<PolicyResponse> {
        if let Some(before) = <Self as Policy<&song_staff::Model>>::global_before(user, action) {
            return Some(before);
        }

        None
    }

    fn authorize(
        user: Option<&CurrentUser>,
        action: &PolicyAction,
        model: Option<&song_staff::Model>,
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

impl SongStaffPolicy {
    fn view_any() -> PolicyResponse {
        PolicyResponse::Allow
    }

    fn view(_user: Option<&CurrentUser>, _staff: &song_staff::Model) -> PolicyResponse {
        PolicyResponse::Allow
    }

    fn create(user: &CurrentUser) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, vec![Roles::Admin]);

        if has_role {
            return PolicyResponse::Allow;
        }

        PolicyResponse::Deny
    }

    fn update(user: &CurrentUser, _staff: &song_staff::Model) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, vec![Roles::Admin]);

        if has_role {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
    }

    fn delete(user: &CurrentUser, _staff: &song_staff::Model) -> PolicyResponse {
        let has_role = has_any_role(&user.roles, vec![Roles::Admin]);

        if has_role {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
    }
}
