use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::admin::announcement;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveAnnouncementInput")]
pub struct SaveAnnouncementInput {
    #[validate(length(
        min = 1,
        max = 65535,
        message = "The content must be between 1 and 65535 characters."
    ))]
    pub content: Option<String>,

    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct SaveAnnouncementAction;

impl SaveAnnouncementAction {
    pub async fn save(
        _ctx: &AppContext,
        _id: Option<u64>,
        _input: SaveAnnouncementInput,
        _user: &CurrentUser,
    ) -> Result<announcement::Model, AppError> {
        todo!()
    }

    pub async fn delete(_ctx: &AppContext, _id: u64, _user: &CurrentUser) -> Result<(), AppError> {
        todo!()
    }
}
