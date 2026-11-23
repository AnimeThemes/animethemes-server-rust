use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::content::song;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveArtistInput")]
pub struct SaveArtistInput {
    #[validate(nested)]
    pub name: Option<ArtistNameInput>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The slug must be between 1 and 255 characters."
    ))]
    pub slug: Option<String>,
}

#[derive(InputObject, Validate)]
#[graphql(name = "ArtistNameInput")]
pub struct ArtistNameInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "The main name must be between 1 and 255 characters."
    ))]
    pub main: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The native name must be between 1 and 255 characters."
    ))]
    pub native: Option<String>,
}

pub struct SaveArtistAction;

impl SaveArtistAction {
    pub async fn save(
        _ctx: &AppContext,
        _id: Option<u64>,
        _input: SaveArtistInput,
        _user: &CurrentUser,
    ) -> Result<song::Model, AppError> {
        todo!()
    }

    pub async fn delete(_ctx: &AppContext, _id: u64, _user: &CurrentUser) -> Result<(), AppError> {
        todo!()
    }
}
