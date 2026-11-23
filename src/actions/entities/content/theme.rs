use async_graphql::InputObject;
use loco_rs::app::AppContext;
use validator::Validate;

use crate::AppError;
use crate::entities::content::theme;
use crate::enums::content::themetype::ThemeType;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveThemeInput")]
pub struct SaveThemeInput {
    pub anime_id: Option<u64>,

    pub r#type: Option<ThemeType>,

    #[validate(range(
        min = 1,
        max = 999,
        message = "The sequence must be between 1 and 999."
    ))]
    pub sequence: Option<i32>,

    pub group_id: Option<u64>,

    pub song_id: Option<u64>,
}

pub struct SaveThemeAction;

impl SaveThemeAction {
    pub async fn save(
        _ctx: &AppContext,
        _id: Option<u64>,
        _input: SaveThemeInput,
        _user: &CurrentUser,
    ) -> Result<theme::Model, AppError> {
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
