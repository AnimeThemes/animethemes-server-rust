use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::content::song;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveSongInput")]
pub struct SaveSongInput {
    #[validate(nested)]
    pub title: Option<SongTitleInput>,
}

#[derive(InputObject, Validate)]
#[graphql(name = "SongTitleInput")]
pub struct SongTitleInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "The romaji title must be between 1 and 255 characters."
    ))]
    pub romaji: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The native title must be between 1 and 255 characters."
    ))]
    pub native: Option<String>,
}

pub struct SaveSongAction;

impl SaveSongAction {
    pub async fn save(
        _ctx: &AppContext,
        _song: Option<song::Model>,
        _input: SaveSongInput,
        _user: &CurrentUser,
    ) -> Result<song::Model, AppError> {
        todo!()
    }

    pub async fn delete(
        _ctx: &AppContext,
        _song: song::Model,
        _user: &CurrentUser,
    ) -> Result<(), AppError> {
        todo!()
    }
}
