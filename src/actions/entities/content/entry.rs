use async_graphql::{InputObject, MaybeUndefined};
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, ModelTrait, TryIntoModel};
use validator::{Validate, ValidationError};

use crate::actions::entities::{nullable_string, optional};
use crate::entities::content::entry;
use crate::events::content::entry_created::EntryCreated;
use crate::events::content::entry_deleting::EntryDeleting;
use crate::events::content::entry_updated::EntryUpdated;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::*;
use crate::traits::soft_delete::SoftDeleteModel;
use crate::{AppError, events};

#[derive(InputObject, Validate)]
#[graphql(name = "SaveEntryInput")]
pub struct SaveEntryInput {
    pub theme_id: Option<u64>,

    #[validate(custom(
        function = "string::validate_length::<1, 100>",
        message = "The episodes must be between 1 and 100 characters."
    ))]
    pub episodes: MaybeUndefined<String>,

    #[validate(custom(
        function = "string::validate_length::<1, 500>",
        message = "The notes must be between 1 and 500 characters."
    ))]
    pub notes: MaybeUndefined<String>,

    pub nsfw: Option<bool>,

    pub spoiler: Option<bool>,

    #[validate(range(min = 1, max = 100, message = "The version must be between 1 and 100."))]
    pub version: Option<i32>,
}

pub struct SaveEntryAction;

impl SaveEntryAction {
    pub async fn save(
        ctx: &AppContext,
        entry: Option<entry::Model>,
        input: SaveEntryInput,
        user: &CurrentUser,
    ) -> Result<entry::Model, AppError> {
        let mut errors = input.validate().err().unwrap_or_default();

        if entry.is_none() {
            if input.theme_id.is_none() {
                errors.add(
                    "themeId",
                    ValidationError::new("required")
                        .with_message("The themeId is required.".into()),
                );
            }

            if input.nsfw.is_none() {
                errors.add(
                    "nsfw",
                    ValidationError::new("required").with_message("The nsfw is required.".into()),
                );
            }

            if input.spoiler.is_none() {
                errors.add(
                    "spoiler",
                    ValidationError::new("required")
                        .with_message("The spoiler is required.".into()),
                );
            }
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        let old_entry = entry.as_ref().cloned();

        let entry = entry::ActiveModel {
            id: optional(entry.as_ref().map(|e| e.id)),
            theme_id: optional(input.theme_id),
            episodes: nullable_string(input.episodes),
            notes: nullable_string(input.notes),
            nsfw: optional(input.nsfw),
            spoiler: optional(input.spoiler),
            version: optional(input.version),
            ..Default::default()
        };

        let entry = entry.save(&ctx.db).await?.try_into_model()?;

        if let Some(old_entry) = old_entry {
            events::dispatch(
                ctx,
                EntryUpdated {
                    old_entry: old_entry,
                    new_entry: entry.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        } else {
            events::dispatch(
                ctx,
                EntryCreated {
                    entry: entry.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        }

        Ok(entry)
    }

    pub async fn delete(
        ctx: &AppContext,
        entry: entry::Model,
        is_force_delete: bool,
        user: &CurrentUser,
    ) -> Result<(), AppError> {
        events::dispatch(
            ctx,
            EntryDeleting {
                entry: entry.clone(),
                is_force_deleting: is_force_delete,
                user: Some(user.clone()),
            },
        )
        .await;

        if is_force_delete {
            entry.delete(&ctx.db).await?;
        } else {
            entry.soft_delete(&ctx.db).await?;
        }

        Ok(())
    }
}
