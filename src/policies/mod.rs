use animethemes_server_rust::entities::auth::role::{self, Roles};
use sea_orm::EntityTrait;

use crate::middlewares::current_user::CurrentUser;

pub mod list;

#[derive(Debug, Clone, Copy)]
pub enum PolicyAction {
    ViewAny,
    View,
    Create,
    Update,
    Delete,
}

pub trait Policy<E, Model>
where
    E: EntityTrait,
{
    fn before(user: Option<&CurrentUser>, _action: &PolicyAction) -> Option<PolicyResponse> {
        if let Some(user) = user
            && has_any_role(&user.roles, &vec![Roles::SuperAdmin])
        {
            return Some(PolicyResponse::Allow);
        }

        None
    }

    fn authorize(
        user: Option<&CurrentUser>,
        action: &PolicyAction,
        model: Option<&Model>,
    ) -> PolicyResponse;

    fn after(
        _user: Option<&CurrentUser>,
        _action: &PolicyAction,
        _model: Option<&Model>,
        _result: &PolicyResponse,
    ) -> Option<PolicyResponse> {
        None
    }

    fn check(
        user: Option<&CurrentUser>,
        action: PolicyAction,
        model: Option<&Model>,
    ) -> PolicyResponse {
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
    _DenyWithMessage(String),
}

impl PolicyResponse {
    pub fn _is_allowed(&self) -> bool {
        matches!(self, Self::Allow)
    }

    pub fn authorize(self) -> Result<(), AppError> {
        match self {
            PolicyResponse::Allow => Ok(()),
            PolicyResponse::Deny => Err(AppError::Forbidden),
            PolicyResponse::DenyAsNotFound => Err(AppError::NotFound),
            PolicyResponse::_DenyWithMessage(message) => {
                Err(AppError::ForbiddenWithMessage(message))
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Unauthenticated")]
    Unauthenticated,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("{0}")]
    ForbiddenWithMessage(String),
}

pub fn has_any_role(user_roles: &[role::Model], roles: &[Roles]) -> bool {
    user_roles.iter().any(|role| {
        role.name
            .parse::<Roles>()
            .is_ok_and(|role| roles.contains(&role))
    })
}
