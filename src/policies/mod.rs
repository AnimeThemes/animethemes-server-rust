use crate::{
    AppError,
    entities::auth::{
        role::{self, Roles},
        sanction::{self, Sanctions},
        user_sanctions,
    },
};

use crate::middlewares::current_user::CurrentUser;

pub mod document;
pub mod list;

#[derive(Debug, Clone, Copy)]
pub enum PolicyAction {
    ViewAny,
    View,
    Create,
    Update,
    Delete,
}

pub trait Policy<T: Copy> {
    fn global_before(user: Option<&CurrentUser>, _action: &PolicyAction) -> Option<PolicyResponse> {
        if let Some(user) = user
            && has_any_role(&user.roles, vec![Roles::Admin])
        {
            return Some(PolicyResponse::Allow);
        }

        None
    }

    fn before(user: Option<&CurrentUser>, action: &PolicyAction) -> Option<PolicyResponse> {
        Self::global_before(user, action)
    }

    fn authorize(
        user: Option<&CurrentUser>,
        action: &PolicyAction,
        model: Option<T>,
    ) -> PolicyResponse;

    fn after(
        _user: Option<&CurrentUser>,
        _action: &PolicyAction,
        _model: Option<T>,
        _result: &PolicyResponse,
    ) -> Option<PolicyResponse> {
        None
    }

    fn check(user: Option<&CurrentUser>, action: PolicyAction, model: Option<T>) -> PolicyResponse {
        if let Some(response) = Self::before(user, &action) {
            return response;
        }

        let result = Self::authorize(user, &action, model);

        if let Some(response) = Self::after(user, &action, model, &result) {
            return response;
        }

        result
    }
}

pub enum PolicyResponse {
    Allow,
    Deny,
    DenyAsNotFound,
    DenyWithMessage(String),
}

impl PolicyResponse {
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allow)
    }

    pub fn authorize(self) -> Result<(), AppError> {
        match self {
            PolicyResponse::Allow => Ok(()),
            PolicyResponse::Deny => Err(AppError::Forbidden),
            PolicyResponse::DenyAsNotFound => Err(AppError::NotFound),
            PolicyResponse::DenyWithMessage(message) => {
                Err(AppError::ForbiddenWithMessage(message))
            }
        }
    }
}

pub fn has_any_role(user_roles: &[role::Model], roles: Vec<Roles>) -> bool {
    user_roles.iter().any(|role| {
        role.name
            .parse::<Roles>()
            .is_ok_and(|role| roles.contains(&role))
    })
}

pub fn get_sanction<'a>(
    user_sanctions: &'a [(user_sanctions::Model, sanction::Model)],
    sanctions: Vec<Sanctions>,
) -> Option<(&'a user_sanctions::Model, &'a sanction::Model)> {
    user_sanctions
        .iter()
        .find(|(_, sanction)| {
            sanction
                .name
                .parse::<Sanctions>()
                .is_ok_and(|parsed| sanctions.contains(&parsed))
        })
        .map(|(user_sanction, sanction)| (user_sanction, sanction))
}
