use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::content::themegroup;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveThemeGroupInput")]
pub struct SaveThemeGroupInput {
    #[validate(length(
        min = 1,
        max = 192,
        message = "The name must be between 1 and 192 characters."
    ))]
    pub name: Option<String>,

    #[validate(length(
        min = 1,
        max = 15,
        message = "The slug must be between 1 and 15 characters."
    ))]
    pub slug: Option<String>,
}

pub struct SaveThemeGroupAction;

impl SaveThemeGroupAction {
    pub async fn save(
        _ctx: &AppContext,
        _group: Option<themegroup::Model>,
        _input: SaveThemeGroupInput,
        _user: &CurrentUser,
    ) -> Result<themegroup::Model, AppError> {
        todo!()
    }

    pub async fn delete(
        _ctx: &AppContext,
        _group: themegroup::Model,
        _user: &CurrentUser,
    ) -> Result<(), AppError> {
        todo!()
    }
}
