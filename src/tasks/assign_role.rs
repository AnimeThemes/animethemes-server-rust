use loco_rs::Error as LocoError;
use loco_rs::prelude::*;

use crate::entities::auth::role;
use crate::entities::auth::user_roles;

pub struct AssignRole;

#[async_trait]
impl Task for AssignRole {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "assign:role".to_string(),
            detail: "Assign a role to a user.\n\
                    Usage: assign:role id:<user_id> role:<role_name>"
                .to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let id = vars
            .cli_arg("id")
            .map_err(|_| LocoError::string("id is required"))?
            .parse::<u64>()
            .map_err(|_| LocoError::string("id must be a positive integer"))?;

        let role = vars
            .cli_arg("role")
            .map_err(|_| LocoError::string("role is required"))?;

        let role = role::Entity::find_by_name(role)
            .one(&app_context.db)
            .await
            .map_err(|_| LocoError::InternalServerError)?
            .ok_or(LocoError::NotFound)?;

        user_roles::ActiveModel {
            role_id: Set(role.id),
            user_id: Set(id),
            ..Default::default()
        }
        .insert(&app_context.db)
        .await
        .map_err(LocoError::DB)?;

        println!("Task AssignRole executed");

        Ok(())
    }
}
