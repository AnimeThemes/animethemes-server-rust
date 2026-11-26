use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::content::studio;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveStudioInput")]
pub struct SaveStudioInput {
    #[validate(length(
        min = 1,
        max = 192,
        message = "The name must be between 1 and 192 characters."
    ))]
    pub name: Option<String>,
}

pub struct SaveStudioAction;

impl SaveStudioAction {
    pub async fn save(
        _ctx: &AppContext,
        _studio: Option<studio::Model>,
        _input: SaveStudioInput,
        _user: &CurrentUser,
    ) -> Result<studio::Model, AppError> {
        todo!()
    }

    pub async fn delete(
        _ctx: &AppContext,
        _studio: studio::Model,
        _user: &CurrentUser,
    ) -> Result<(), AppError> {
        todo!()
    }
}
