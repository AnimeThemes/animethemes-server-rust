use std::str::FromStr;

use crate::{
    entities::{
        auth::role::{self, Roles},
        document::{page, page_roles},
    },
    enums::document::pageroletype::PageRoleType,
};

use crate::{
    middlewares::current_user::CurrentUser,
    policies::{Policy, PolicyAction, PolicyResponse, has_any_role},
};

pub struct PagePolicy;

impl Policy<(&page::Model, &Vec<(page_roles::Model, role::Model)>)> for PagePolicy {
    fn authorize(
        user: Option<&CurrentUser>,
        action: &PolicyAction,
        model: Option<(&page::Model, &Vec<(page_roles::Model, role::Model)>)>,
    ) -> PolicyResponse {
        match action {
            PolicyAction::ViewAny => Self::view_any(),
            PolicyAction::View => Self::view(user, model.expect("Model expected")),
            _ => PolicyResponse::Deny,
        }
    }
}

impl PagePolicy {
    fn view_any() -> PolicyResponse {
        PolicyResponse::Allow
    }

    fn view(
        user: Option<&CurrentUser>,
        models: (&page::Model, &Vec<(page_roles::Model, role::Model)>),
    ) -> PolicyResponse {
        let (_, page_roles) = models;

        if !page_roles
            .iter()
            .any(|(pivot, _)| pivot.r#type == PageRoleType::Viewer)
        {
            return PolicyResponse::Allow;
        }

        if let Some(user) = user
            && has_any_role(
                &user.roles,
                page_roles
                    .iter()
                    .filter_map(|(_, role)| Roles::from_str(&role.name).ok())
                    .collect::<Vec<_>>(),
            )
        {
            return PolicyResponse::Allow;
        }

        PolicyResponse::DenyAsNotFound
    }
}
