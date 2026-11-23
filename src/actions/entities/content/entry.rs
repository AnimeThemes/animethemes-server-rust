use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::content::entry;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveEntryInput")]
pub struct SaveEntryInput {
    pub theme_id: Option<u64>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "The episodes must be between 1 and 100 characters."
    ))]
    pub episodes: Option<String>,

    #[validate(length(
        min = 1,
        max = 500,
        message = "The notes must be between 1 and 500 characters."
    ))]
    pub notes: Option<String>,

    pub nsfw: Option<bool>,

    pub spoiler: Option<bool>,

    #[validate(range(min = 1, max = 100, message = "The version must be between 1 and 100."))]
    pub version: Option<i32>,
}

pub struct SaveEntryAction;

impl SaveEntryAction {
    pub async fn save(
        _ctx: &AppContext,
        _id: Option<u64>,
        _input: SaveEntryInput,
        _user: &CurrentUser,
    ) -> Result<entry::Model, AppError> {
        todo!()
    }

    pub async fn delete(
        _ctx: &AppContext,
        _ids: Vec<u64>,
        _user: &CurrentUser,
    ) -> Result<(), AppError> {
        todo!()
    }
}
