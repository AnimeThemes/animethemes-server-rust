use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::admin::featuredtheme;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveFeaturedThemeInput")]
pub struct SaveFeaturedThemeInput {
    pub start_at: Option<time::OffsetDateTime>,
    pub end_at: Option<time::OffsetDateTime>,
    pub entry_id: Option<u64>,
    pub video_id: Option<u64>,
    pub user_id: Option<u64>,
}

pub struct SaveFeaturedThemeAction;

impl SaveFeaturedThemeAction {
    pub async fn save(
        _ctx: &AppContext,
        _id: Option<u64>,
        _input: SaveFeaturedThemeInput,
        _user: &CurrentUser,
    ) -> Result<featuredtheme::Model, AppError> {
        todo!()
    }

    pub async fn delete(_ctx: &AppContext, _id: u64, _user: &CurrentUser) -> Result<(), AppError> {
        todo!()
    }
}
