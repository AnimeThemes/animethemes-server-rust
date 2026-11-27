use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, ModelTrait, TryIntoModel};
use validator::Validate;

use crate::actions::entities::{optional, optional_string};
use crate::entities::content::series;
use crate::events::content::series_created::SeriesCreated;
use crate::events::content::series_deleting::SeriesDeleting;
use crate::events::content::series_updated::SeriesUpdated;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::validate_required_string;
use crate::traits::soft_delete::SoftDeleteModel;
use crate::{AppError, events};

#[derive(InputObject, Validate)]
#[graphql(name = "SaveSeriesInput")]
pub struct SaveSeriesInput {
    #[validate(nested)]
    pub title: Option<SeriesTitleInput>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The slug must be between 1 and 255 characters."
    ))]
    pub slug: Option<String>,
}

#[derive(InputObject, Validate, Default)]
#[graphql(name = "SeriesTitleInput")]
pub struct SeriesTitleInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "The romaji title must be between 1 and 255 characters."
    ))]
    pub romaji: Option<String>,
}

pub struct SaveSeriesAction;

impl SaveSeriesAction {
    pub async fn save(
        ctx: &AppContext,
        series: Option<series::Model>,
        input: SaveSeriesInput,
        user: &CurrentUser,
    ) -> Result<series::Model, AppError> {
        let mut errors = input.validate().err().unwrap_or_default();

        if series.is_none() {
            validate_required_string(
                &mut errors,
                "title.romaji",
                input.title.as_ref().and_then(|t| t.romaji.as_deref()),
                "The romaji title is required.",
            );

            validate_required_string(
                &mut errors,
                "slug",
                input.slug.as_deref(),
                "The slug is required.",
            );
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        let title = input.title.unwrap_or_default();

        let old_series = series.as_ref().cloned();

        let series = series::ActiveModel {
            id: optional(series.as_ref().map(|s| s.id)),
            title: optional_string(title.romaji),
            slug: optional_string(input.slug),
            ..Default::default()
        };

        let series = series.save(&ctx.db).await?.try_into_model()?;

        if let Some(old_series) = old_series {
            events::dispatch(
                ctx,
                SeriesUpdated {
                    old_series: old_series,
                    new_series: series.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        } else {
            events::dispatch(
                ctx,
                SeriesCreated {
                    series: series.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        }

        Ok(series)
    }

    pub async fn delete(
        ctx: &AppContext,
        series: series::Model,
        is_force_delete: bool,
        user: &CurrentUser,
    ) -> Result<(), AppError> {
        events::dispatch(
            ctx,
            SeriesDeleting {
                series: series.clone(),
                is_force_deleting: is_force_delete,
                user: Some(user.clone()),
            },
        )
        .await;

        if is_force_delete {
            series.delete(&ctx.db).await?;
        } else {
            series.soft_delete(&ctx.db).await?;
        }

        Ok(())
    }
}
