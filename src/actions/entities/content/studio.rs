use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, ModelTrait, TryIntoModel};
use validator::Validate;

use crate::actions::entities::{optional, optional_string};
use crate::entities::content::studio;
use crate::events::content::studio_created::StudioCreated;
use crate::events::content::studio_deleting::StudioDeleting;
use crate::events::content::studio_updated::StudioUpdated;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::validate_required_string;
use crate::traits::soft_delete::SoftDeleteModel;
use crate::{AppError, events};

#[derive(InputObject, Validate)]
#[graphql(name = "SaveStudioInput")]
pub struct SaveStudioInput {
    #[validate(length(
        min = 1,
        max = 192,
        message = "The name must be between 1 and 192 characters."
    ))]
    pub name: Option<String>,

    #[validate(length(
        min = 1,
        max = 192,
        message = "The slug must be between 1 and 192 characters."
    ))]
    pub slug: Option<String>,
}

pub struct SaveStudioAction;

impl SaveStudioAction {
    pub async fn save(
        ctx: &AppContext,
        studio: Option<studio::Model>,
        input: SaveStudioInput,
        user: &CurrentUser,
    ) -> Result<studio::Model, AppError> {
        let mut errors = input.validate().err().unwrap_or_default();

        if studio.is_none() {
            validate_required_string(
                &mut errors,
                "name",
                input.name.as_deref(),
                "The name is required.",
            );
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        let old_studio = studio.as_ref().cloned();

        let studio = studio::ActiveModel {
            id: optional(studio.as_ref().map(|s| s.id)),
            name: optional_string(input.name),
            slug: optional_string(input.slug),
            ..Default::default()
        };

        let studio = studio.save(&ctx.db).await?.try_into_model()?;

        if let Some(old_studio) = old_studio {
            events::dispatch(
                ctx,
                StudioUpdated {
                    old_studio: old_studio,
                    new_studio: studio.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        } else {
            events::dispatch(
                ctx,
                StudioCreated {
                    studio: studio.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        }

        Ok(studio)
    }

    pub async fn delete(
        ctx: &AppContext,
        studio: studio::Model,
        is_force_delete: bool,
        user: &CurrentUser,
    ) -> Result<(), AppError> {
        events::dispatch(
            ctx,
            StudioDeleting {
                studio: studio.clone(),
                is_force_deleting: is_force_delete,
                user: Some(user.clone()),
            },
        )
        .await;

        if is_force_delete {
            studio.delete(&ctx.db).await?;
        } else {
            studio.soft_delete(&ctx.db).await?;
        }

        Ok(())
    }
}
