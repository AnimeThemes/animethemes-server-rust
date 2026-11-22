use anyhow::Context;
use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::AppError;
use crate::entities::content::anime;
use crate::enums::content::animeformat::AnimeFormat;
use crate::enums::content::animeseason::AnimeSeason;
use crate::middlewares::current_user::CurrentUser;
use crate::rules::validation_error::ValidationError;
use crate::typesense::client::typesense;
use crate::typesense::documents::anime_document::{AnimeDocument, build_anime_documents};

#[derive(InputObject)]
#[graphql(name = "CreateAnimeInput")]
pub struct CreateAnimeActionParameters {
    title: String,
    title_english: Option<String>,
    title_native: Option<String>,
    season: Option<AnimeSeason>,
    year: Option<i32>,
    format: Option<AnimeFormat>,
    synopsis: Option<String>,
}

pub struct CreateAnimeAction;

impl CreateAnimeAction {
    fn validate(params: &CreateAnimeActionParameters) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let mut title_errors = Vec::new();
        let mut title_english_errors = Vec::new();
        let mut title_native_errors = Vec::new();
        let mut year_errors = Vec::new();
        let mut synopsis_errors = Vec::new();

        if !(1usize..=255).contains(&params.title.chars().count()) {
            title_errors.push("The title must be between 1 and 255 characters.");
        }

        if let Some(title_english) = &params.title_english {
            if !(1usize..=255).contains(&title_english.chars().count()) {
                title_english_errors
                    .push("The english title must be between 1 and 255 characters.");
            }
        }

        if let Some(title_native) = &params.title_native {
            if !(1usize..=255).contains(&title_native.chars().count()) {
                title_native_errors.push("The native title must be between 1 and 255 characters.");
            }
        }

        if let Some(year) = &params.year {
            if !(1900..=2100).contains(year) {
                year_errors.push("The year must be between 1900 and 2100.");
            }
        }

        if let Some(synopsis) = &params.synopsis {
            if !(1usize..=1000).contains(&synopsis.chars().count()) {
                synopsis_errors.push("The synopsis must be between 1 and 1000 characters.");
            }
        }

        if !title_errors.is_empty() {
            errors.push(ValidationError::new("title", title_errors));
        }

        if !title_english_errors.is_empty() {
            errors.push(ValidationError::new("title_english", title_english_errors));
        }

        if !title_native_errors.is_empty() {
            errors.push(ValidationError::new("title_native", title_native_errors));
        }

        if !year_errors.is_empty() {
            errors.push(ValidationError::new("year", year_errors));
        }

        if !synopsis_errors.is_empty() {
            errors.push(ValidationError::new("synopsis", synopsis_errors));
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn create(
        ctx: &AppContext,
        params: CreateAnimeActionParameters,
        _user: &CurrentUser,
    ) -> Result<anime::Model, AppError> {
        Self::validate(&params)?;

        let anime = anime::ActiveModel {
            title: Set(params.title),
            title_english: Set(params.title_english),
            title_native: Set(params.title_native),
            season: Set(params.season),
            year: Set(params.year),
            format: Set(params.format),
            synopsis: Set(params.synopsis),
            ..Default::default()
        };

        let anime = anime.insert(&ctx.db).await?;

        Self::update_search(&ctx.db, &anime).await?;

        Ok(anime)
    }

    async fn update_search(db: &DatabaseConnection, anime: &anime::Model) -> Result<(), AppError> {
        let document = build_anime_documents(vec![anime.clone()], db)
            .await
            .map_err(AppError::internal)?
            .into_iter()
            .next()
            .context("Failed")
            .map_err(AppError::internal)?;

        let typesense = typesense();

        typesense
            .collection::<AnimeDocument>()
            .documents()
            .create(&document, None)
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
