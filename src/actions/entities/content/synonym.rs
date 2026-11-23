use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::content::synonym;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveSynonymInput")]
pub struct SaveSynonymInput {
    pub anime_id: Option<u64>,
    pub artist_id: Option<u64>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The text must be between 1 and 255 characters."
    ))]
    pub text: Option<String>,

    #[validate(length(
        min = 1,
        max = 50,
        message = "The language must be between 1 and 50 characters."
    ))]
    pub language: Option<String>,
}

pub struct SaveSynonymAction;

impl SaveSynonymAction {
    pub async fn save(
        _ctx: &AppContext,
        _id: Option<u64>,
        _input: SaveSynonymInput,
        _user: &CurrentUser,
    ) -> Result<synonym::Model, AppError> {
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
