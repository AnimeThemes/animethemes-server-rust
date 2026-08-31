use chrono::{Duration, Utc};
use loco_rs::Error as LocoError;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::entities::auth::password_reset_tokens;

pub struct ClearResetPasswords;

#[async_trait]
impl Task for ClearResetPasswords {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "clear:reset-passwords".to_string(),
            detail: "Clear the expired reset password tokens".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        password_reset_tokens::Entity::delete_many()
            .filter(password_reset_tokens::Column::CreatedAt.lt(Utc::now() - Duration::hours(1)))
            .exec(&app_context.db)
            .await
            .map_err(|_| LocoError::InternalServerError)?;

        tracing::info!("Task ClearResetPasswords executed");

        Ok(())
    }
}
