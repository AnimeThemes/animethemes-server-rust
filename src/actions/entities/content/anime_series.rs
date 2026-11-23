use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::content::anime_series;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveAnimeSeriesInput")]
pub struct SaveAnimeSeriesInput {
    pub anime_id: Option<u64>,
    pub series_id: Option<u64>,
}

pub struct SaveAnimeSeriesAction;

impl SaveAnimeSeriesAction {
    pub async fn save(
        _ctx: &AppContext,
        _id: Option<u64>,
        _input: SaveAnimeSeriesInput,
        _user: &CurrentUser,
    ) -> Result<anime_series::Model, AppError> {
        todo!()
    }

    pub async fn delete(
        _ctx: &AppContext,
        _anime_id: u64,
        _series_id: u64,
        _user: &CurrentUser,
    ) -> Result<(), AppError> {
        todo!()
    }
}
