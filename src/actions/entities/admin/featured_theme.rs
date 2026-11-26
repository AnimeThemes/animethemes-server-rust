use async_graphql::{InputObject, MaybeUndefined};
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, ModelTrait, TryIntoModel};
use validator::{Validate, ValidationError};

use crate::AppError;
use crate::actions::entities::{nullable, optional};
use crate::entities::admin::featuredtheme;
use crate::middlewares::current_user::CurrentUser;

#[derive(InputObject, Validate)]
#[graphql(name = "SaveFeaturedThemeInput")]
pub struct SaveFeaturedThemeInput {
    pub start_at: Option<time::OffsetDateTime>,
    pub end_at: Option<time::OffsetDateTime>,
    pub entry_id: MaybeUndefined<u64>,
    pub video_id: MaybeUndefined<u64>,
    pub user_id: MaybeUndefined<u64>,
}

pub struct SaveFeaturedThemeAction;

impl SaveFeaturedThemeAction {
    pub async fn save(
        ctx: &AppContext,
        featuredtheme: Option<featuredtheme::Model>,
        input: SaveFeaturedThemeInput,
        _user: &CurrentUser,
    ) -> Result<featuredtheme::Model, AppError> {
        let mut errors = input.validate().err().unwrap_or_default();

        let start_at = input
            .start_at
            .or(featuredtheme.as_ref().map(|t| t.start_at));

        let end_at = input.end_at.or(featuredtheme.as_ref().map(|t| t.end_at));

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

        let model = featuredtheme::ActiveModel {
            id: optional(featuredtheme.as_ref().map(|a| a.id)),
            start_at: optional(input.start_at),
            end_at: optional(input.end_at),
            entry_id: nullable(input.entry_id),
            video_id: nullable(input.video_id),
            user_id: nullable(input.user_id),
            ..Default::default()
        };

        let model = model.save(&ctx.db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn delete(
        ctx: &AppContext,
        featuredtheme: featuredtheme::Model,
        _user: &CurrentUser,
    ) -> Result<(), AppError> {
        featuredtheme.delete(&ctx.db).await?;

        Ok(())
    }
}
