use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::content::series;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveSeriesInput")]
pub struct SaveSeriesInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "The title must be between 1 and 255 characters."
    ))]
    pub title: Option<String>,
}

pub struct SaveSeriesAction;

impl SaveSeriesAction {
    pub async fn save(
        _ctx: &AppContext,
        _id: Option<u64>,
        _input: SaveSeriesInput,
        _user: &CurrentUser,
    ) -> Result<series::Model, AppError> {
        todo!()
    }

    pub async fn delete(_ctx: &AppContext, _id: u64, _user: &CurrentUser) -> Result<(), AppError> {
        todo!()
    }
}
