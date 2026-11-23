use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::content::resource;
use crate::enums::content::resourcesite::ResourceSite;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveResourceInput")]
pub struct SaveResourceInput {
    pub anime_id: Option<u64>,
    pub artist_id: Option<u64>,
    pub entry_id: Option<u64>,
    pub song_id: Option<u64>,
    pub studio_id: Option<u64>,

    pub external_id: Option<i32>,

    pub link: Option<String>,

    pub site: Option<ResourceSite>,

    #[validate(length(
        min = 1,
        max = 50,
        message = "The as must be between 1 and 50 characters."
    ))]
    pub r#as: Option<String>,
}

pub struct SaveResourceAction;

impl SaveResourceAction {
    pub async fn save(
        _ctx: &AppContext,
        _id: Option<u64>,
        _input: SaveResourceInput,
        _user: &CurrentUser,
    ) -> Result<resource::Model, AppError> {
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
