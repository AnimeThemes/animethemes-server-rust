use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::document::page;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SavePageInput")]
pub struct SavePageInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "The name must be between 1 and 255 characters."
    ))]
    pub name: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The slug must be between 1 and 255 characters."
    ))]
    pub slug: Option<String>,

    #[validate(length(
        min = 1,
        max = 65535,
        message = "The body must be between 1 and 65535 characters."
    ))]
    pub body: Option<String>,

    pub previous_id: Option<u64>,
    pub next_id: Option<u64>,
}

pub struct SavePageAction;

impl SavePageAction {
    pub async fn save(
        _ctx: &AppContext,
        _page: Option<page::Model>,
        _input: SavePageInput,
        _user: &CurrentUser,
    ) -> Result<page::Model, AppError> {
        todo!()
    }

    pub async fn delete(
        _ctx: &AppContext,
        _page: page::Model,
        _user: &CurrentUser,
    ) -> Result<(), AppError> {
        todo!()
    }
}
