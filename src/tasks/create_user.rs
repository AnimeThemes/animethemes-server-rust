use loco_rs::Error as LocoError;
use loco_rs::prelude::*;

use crate::AppError;
use crate::actions::auth::register::Register;
use crate::actions::auth::register::RegisterParameters;

pub struct CreateUser;

#[async_trait]
impl Task for CreateUser {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "create:user".to_string(),
            detail: "Create a user with the given data\n\
                    Usage: create:user name:<name> email:<email> password:<password>"
                .to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let name = vars
            .cli_arg("name")
            .map_err(|_| LocoError::string("name is required"))?;
        let email = vars
            .cli_arg("email")
            .map_err(|_| LocoError::string("email is required"))?;
        let password = vars
            .cli_arg("password")
            .map_err(|_| LocoError::string("password is required"))?;

        Register::register(
            &app_context.db,
            RegisterParameters {
                name: name.to_string(),
                email: email.to_string(),
                password: password.to_string(),
                password_confirmation: password.to_string(),
                terms: true,
            },
        )
        .await
        .map_err(|e| <AppError as Into<LocoError>>::into(e))?;

        tracing::info!("Task CreateUser executed");

        Ok(())
    }
}
