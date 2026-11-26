use loco_rs::app::AppContext;

use crate::AppError;
use crate::actions::auth::verify_email::VerifyEmail;
use crate::events::auth::user_registered::UserRegistered;

pub async fn handle(_ctx: &AppContext, event: &UserRegistered) -> Result<(), AppError> {
    VerifyEmail::send_verification_email(&event.user).await?;

    Ok(())
}
