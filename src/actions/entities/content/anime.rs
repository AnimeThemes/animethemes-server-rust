use async_graphql::{InputObject, MaybeUndefined};
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, ModelTrait, TryIntoModel};
use validator::{Validate, ValidationError};

use crate::AppError;
use crate::actions::entities::{nullable, optional};
use crate::entities::content::anime;
use crate::enums::content::animeformat::AnimeFormat;
use crate::enums::content::animeseason::AnimeSeason;
use crate::initializers::typesense::typesense;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::*;
use crate::services::search_indexer::SearchIndexer;
use crate::typesense::documents::anime_document::AnimeDocument;

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
        _user: &CurrentUser,
    ) -> Result<anime::Model, AppError> {
        let mut errors = params.validate().err().unwrap_or_default();

        if anime.is_none() {
            if params
                .title
                .as_ref()
                .and_then(|t| t.romaji.as_deref())
                .is_none_or(|v| v.trim().is_empty())
            {
                errors.add(
                    "title",
                    ValidationError::new("required")
                        .with_message("The romaji title is required.".into()),
                );
            }

            if params.slug.as_deref().is_none_or(|v| v.trim().is_empty()) {
                errors.add(
                    "slug",
                    ValidationError::new("required").with_message("The slug is required.".into()),
                );
            }
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        let title = params.title.unwrap_or_default();

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

        SearchIndexer::reindex_anime(&ctx.db, anime.clone()).await?;

        Ok(anime)
    }

    pub async fn delete(
        ctx: &AppContext,
        anime: anime::Model,
        _user: &CurrentUser,
    ) -> Result<(), AppError> {
        anime.clone().delete(&ctx.db).await?;

        let typesense = typesense();

        typesense
            .collection::<AnimeDocument>()
            .document(anime.id.to_string())
            .delete()
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
