use async_graphql::{Context, Object, SimpleObject};

use crate::{
    AppError,
    middlewares::current_user::CurrentUser,
    policies::{Policy, PolicyAction, list::playlist::PlaylistPolicy},
};

#[derive(SimpleObject)]
pub struct PermissionsResult {
    /// Whether the permission check was successful.
    pub allow: bool,
    /// The reason for the permission check failure, if any.
    pub reason: Option<String>,
}

pub struct Permissions;

#[Object]
impl Permissions {
    async fn can_create_playlist(&self, ctx: &Context<'_>) -> PermissionsResult {
        let Some(user) = ctx.data_opt::<CurrentUser>() else {
            return PermissionsResult {
                allow: false,
                reason: Some(AppError::Unauthenticated.to_string()),
            };
        };

        match PlaylistPolicy::check(Some(&user), PolicyAction::Create, None).authorize() {
            Ok(_) => PermissionsResult {
                allow: true,
                reason: None,
            },
            Err(err) => PermissionsResult {
                allow: false,
                reason: Some(err.to_string()),
            },
        }
    }
}
