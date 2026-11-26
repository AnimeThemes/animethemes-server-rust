use loco_rs::app::AppContext;

use crate::AppError;
use crate::actions::auth::roles::RoleAction;
use crate::entities::auth::role::Roles;
use crate::events::auth::user_verified::UserVerified;

pub async fn assign_verified_role(ctx: &AppContext, event: &UserVerified) -> Result<(), AppError> {
    RoleAction::assign_role(&ctx, &event.user, Roles::Verified).await?;

    Ok(())
}
