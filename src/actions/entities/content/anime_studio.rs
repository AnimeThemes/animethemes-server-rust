use async_graphql::InputObject;
use loco_rs::app::AppContext;

use crate::AppError;
use crate::entities::content::anime_studios;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject)]
#[graphql(name = "SaveAnimeStudioInput")]
pub struct SaveAnimeStudioInput {
    pub anime_id: Option<u64>,
    pub studio_id: Option<u64>,
}

pub struct SaveAnimeStudioAction;

impl SaveAnimeStudioAction {
    pub async fn save(
        _ctx: &AppContext,
        _id: Option<u64>,
        _input: SaveAnimeStudioInput,
        _user: &CurrentUser,
    ) -> Result<anime_studios::Model, AppError> {
        todo!()
    }

    pub async fn delete(
        _ctx: &AppContext,
        _anime_id: u64,
        _studio_id: u64,
        _user: &CurrentUser,
    ) -> Result<(), AppError> {
        todo!()
    }
}
