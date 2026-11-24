use async_graphql::{InputObject, MaybeUndefined};
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, ModelTrait, TryIntoModel};
use validator::{Validate, ValidationError};

use crate::AppError;
use crate::actions::entities::{nullable, optional};
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

    pub start_at: MaybeUndefined<chrono::DateTime<chrono::Utc>>,
    pub end_at: MaybeUndefined<chrono::DateTime<chrono::Utc>>,
}

pub struct SaveAnnouncementAction;

impl SaveAnnouncementAction {
    pub async fn save(
        ctx: &AppContext,
        announcement: Option<announcement::Model>,
        input: SaveAnnouncementInput,
        _user: &CurrentUser,
    ) -> Result<announcement::Model, AppError> {
        let mut errors = input.validate().err().unwrap_or_default();

        if announcement.is_none() && input.content.is_none() {
            errors.add(
                "content",
                ValidationError::new("required").with_message("The content is required.".into()),
            );
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        let model = announcement::ActiveModel {
            id: optional(announcement.as_ref().map(|a| a.id)),
            content: optional(input.content),
            start_at: nullable(input.start_at),
            end_at: nullable(input.end_at),
            ..Default::default()
        };

        let model = model.save(&ctx.db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn delete(
        ctx: &AppContext,
        announcement: announcement::Model,
        _user: &CurrentUser,
    ) -> Result<(), AppError> {
        announcement.delete(&ctx.db).await?;

        Ok(())
    }
}
