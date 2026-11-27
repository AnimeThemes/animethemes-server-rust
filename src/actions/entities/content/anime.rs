use async_graphql::{InputObject, MaybeUndefined};
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, ModelTrait, TryIntoModel};
use validator::Validate;

use crate::actions::entities::{nullable, optional};
use crate::entities::content::anime;
use crate::enums::content::animeformat::AnimeFormat;
use crate::enums::content::animeseason::AnimeSeason;
use crate::events::content::anime_created::AnimeCreated;
use crate::events::content::anime_deleting::AnimeDeleting;
use crate::events::content::anime_updated::AnimeUpdated;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::*;
use crate::traits::soft_delete::SoftDeleteModel;
use crate::{AppError, events};

#[derive(InputObject, Validate)]
#[graphql(name = "SaveAnimeInput")]
pub struct SaveAnimeInput {
    #[validate(nested)]
    title: Option<AnimeTitleInput>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The slug must be between 1 and 255 characters."
    ))]
    slug: Option<String>,

    season: MaybeUndefined<AnimeSeason>,

    #[validate(custom(
        function = "integers::validate_range::<1900, 2100>",
        message = "The year must be between 1900 and 2100."
    ))]
    year: MaybeUndefined<i32>,

    format: MaybeUndefined<AnimeFormat>,

    #[validate(custom(
        function = "string::validate_length::<1, 1000>",
        message = "The synopsis must be between 1 and 1000 characters."
    ))]
    synopsis: MaybeUndefined<String>,
}

#[derive(InputObject, Validate, Default)]
#[graphql(name = "AnimeTitleInput")]
pub struct AnimeTitleInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "The romaji title must be between 1 and 255 characters."
    ))]
    pub romaji: Option<String>,

    #[validate(custom(
        function = "string::validate_length::<1, 255>",
        message = "The english title must be between 1 and 255 characters."
    ))]
    pub english: MaybeUndefined<String>,

    #[validate(custom(
        function = "string::validate_length::<1, 255>",
        message = "The native title must be between 1 and 255 characters."
    ))]
    pub native: MaybeUndefined<String>,
}

pub struct SaveAnimeAction;

impl SaveAnimeAction {
    pub async fn save(
        ctx: &AppContext,
        anime: Option<anime::Model>,
        params: SaveAnimeInput,
        user: &CurrentUser,
    ) -> Result<anime::Model, AppError> {
        let mut errors = params.validate().err().unwrap_or_default();

        if anime.is_none() {
            validate_required_string(
                &mut errors,
                "title.romaji",
                params.title.as_ref().and_then(|t| t.romaji.as_deref()),
                "The romaji title is required.",
            );

            validate_required_string(
                &mut errors,
                "slug",
                params.slug.as_deref(),
                "The slug is required.",
            );
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        let title = params.title.unwrap_or_default();

        let old_anime = anime.as_ref().cloned();

        let anime = anime::ActiveModel {
            id: optional(anime.as_ref().map(|a| a.id)),
            title: optional(title.romaji),
            title_english: nullable(title.english),
            title_native: nullable(title.native),
            slug: optional(params.slug),
            season: nullable(params.season),
            year: nullable(params.year),
            format: nullable(params.format),
            synopsis: nullable(params.synopsis),
            ..Default::default()
        };

        let anime = anime.save(&ctx.db).await?.try_into_model()?;

        if let Some(old_anime) = old_anime {
            events::dispatch(
                ctx,
                AnimeUpdated {
                    old_anime: old_anime,
                    new_anime: anime.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        } else {
            events::dispatch(
                ctx,
                AnimeCreated {
                    anime: anime.clone(),
                    user: Some(user.clone()),
                },
            )
            .await;
        }

        Ok(anime)
    }

    pub async fn delete(
        ctx: &AppContext,
        anime: anime::Model,
        is_force_delete: bool,
        user: &CurrentUser,
    ) -> Result<(), AppError> {
        events::dispatch(
            ctx,
            AnimeDeleting {
                anime: anime.clone(),
                is_force_deleting: is_force_delete,
                user: Some(user.clone()),
            },
        )
        .await;

        if is_force_delete {
            anime.delete(&ctx.db).await?;
        } else {
            anime.soft_delete(&ctx.db).await?;
        }

        Ok(())
    }
}
