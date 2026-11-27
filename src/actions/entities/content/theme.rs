use async_graphql::{InputObject, MaybeUndefined};
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, ModelTrait, TryIntoModel};
use validator::{Validate, ValidationError};

use crate::actions::entities::{nullable, optional};
use crate::entities::content::theme;
use crate::enums::content::themetype::ThemeType;
use crate::events::content::theme_created::ThemeCreated;
use crate::events::content::theme_deleting::ThemeDeleting;
use crate::events::content::theme_updated::ThemeUpdated;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::*;
use crate::traits::soft_delete::SoftDeleteModel;
use crate::{AppError, events};

#[derive(InputObject, Validate)]
#[graphql(name = "SaveThemeInput")]
pub struct SaveThemeInput {
    pub anime_id: Option<u64>,

    pub r#type: Option<ThemeType>,

    #[validate(custom(
        function = "integers::validate_range::<1, 999>",
        message = "The sequence must be between 1 and 999."
    ))]
    pub sequence: MaybeUndefined<i32>,

    pub group_id: MaybeUndefined<u64>,

    pub song_id: MaybeUndefined<u64>,
}

pub struct SaveThemeAction;

impl SaveThemeAction {
    pub async fn save(
        ctx: &AppContext,
        theme: Option<theme::Model>,
        input: SaveThemeInput,
        user: &CurrentUser,
    ) -> Result<theme::Model, AppError> {
        let mut errors = input.validate().err().unwrap_or_default();

        if theme.is_none() {
            if input.anime_id.is_none() {
                errors.add(
                    "animeId",
                    ValidationError::new("required")
                        .with_message("The animeId is required.".into()),
                );
            }

            if input.r#type.is_none() {
                errors.add(
                    "type",
                    ValidationError::new("required").with_message("The type is required.".into()),
                );
            }
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        let old_theme = theme.as_ref().cloned();

        let theme = theme::ActiveModel {
            id: optional(theme.as_ref().map(|a| a.id)),
            anime_id: optional(input.anime_id),
            r#type: optional(input.r#type),
            sequence: nullable(input.sequence),
            group_id: nullable(input.group_id),
            song_id: nullable(input.song_id),
            ..Default::default()
        };

        let theme = theme.save(&ctx.db).await?.try_into_model()?;

        if let Some(old_theme) = old_theme {
            events::dispatch(
                ctx,
                ThemeUpdated {
                    old_theme: old_theme,
                    new_theme: theme.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        } else {
            events::dispatch(
                ctx,
                ThemeCreated {
                    theme: theme.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        }

        Ok(theme)
    }

    pub async fn delete(
        ctx: &AppContext,
        theme: theme::Model,
        is_force_delete: bool,
        user: &CurrentUser,
    ) -> Result<(), AppError> {
        events::dispatch(
            ctx,
            ThemeDeleting {
                theme: theme.clone(),
                is_force_deleting: is_force_delete,
                user: Some(user.clone()),
            },
        )
        .await;

        if is_force_delete {
            theme.delete(&ctx.db).await?;
        } else {
            theme.soft_delete(&ctx.db).await?;
        }

        Ok(())
    }
}
