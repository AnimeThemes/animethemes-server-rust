use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::content::artist_members;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveArtistMemberInput")]
pub struct SaveArtistMemberInput {
    pub artist_id: Option<u64>,
    pub member_id: Option<u64>,

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

    #[validate(length(
        min = 1,
        max = 192,
        message = "The notes must be between 1 and 192 characters."
    ))]
    pub notes: Option<String>,
}

pub struct SaveArtistMemberAction;

impl SaveArtistMemberAction {
    pub async fn save(
        _ctx: &AppContext,
        _id: Option<u64>,
        _input: SaveArtistMemberInput,
        _user: &CurrentUser,
    ) -> Result<artist_members::Model, AppError> {
        todo!()
    }

    pub async fn delete(
        _ctx: &AppContext,
        _artist_id: u64,
        _member_id: u64,
        _user: &CurrentUser,
    ) -> Result<(), AppError> {
        todo!()
    }
}
