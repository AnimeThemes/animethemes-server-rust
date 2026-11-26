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

    pub start_at: MaybeUndefined<time::OffsetDateTime>,
    pub end_at: MaybeUndefined<time::OffsetDateTime>,
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

        let start_at = match input.start_at {
            MaybeUndefined::Undefined => announcement.as_ref().and_then(|t| t.start_at),
            MaybeUndefined::Null => None,
            MaybeUndefined::Value(value) => Some(value),
        };

        let end_at = match input.end_at {
            MaybeUndefined::Undefined => announcement.as_ref().and_then(|t| t.end_at),
            MaybeUndefined::Null => None,
            MaybeUndefined::Value(value) => Some(value),
        };

        if let (Some(start_at), Some(end_at)) = (start_at, end_at)
            && start_at > end_at
        {
            let error = ValidationError::new("")
                .with_message("The start_at must be before the end_at".into());

            errors.add("start_at", error.clone());
            errors.add("end_at", error);
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
