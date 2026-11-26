use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::content::song_staff;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveSongStaffInput")]
pub struct SaveSongStaffInput {
    pub song_id: Option<u64>,
    pub artist_id: Option<u64>,
    pub member_id: Option<u64>,

    #[validate(length(
        min = 1,
        max = 192,
        message = "The role must be between 1 and 192 characters."
    ))]
    pub role: Option<String>,

    #[validate(length(
        min = 1,
        max = 192,
        message = "The alias must be between 1 and 192 characters."
    ))]
    pub alias: Option<String>,

    #[validate(length(
        min = 1,
        max = 192,
        message = "The as must be between 1 and 192 characters."
    ))]
    pub r#as: Option<String>,
}

pub struct SaveSongStaffAction;

impl SaveSongStaffAction {
    pub async fn save(
        _ctx: &AppContext,
        _songstaff: Option<song_staff::Model>,
        _input: SaveSongStaffInput,
        _user: &CurrentUser,
    ) -> Result<song_staff::Model, AppError> {
        todo!()
    }

    pub async fn delete(
        _ctx: &AppContext,
        _songstaff: song_staff::Model,
        _user: &CurrentUser,
    ) -> Result<(), AppError> {
        todo!()
    }
}
