use anyhow::Context;
use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use validator::Validate;

use crate::AppError;
use crate::entities::content::anime;
use crate::enums::content::animeformat::AnimeFormat;
use crate::enums::content::animeseason::AnimeSeason;
use crate::middlewares::current_user::CurrentUser;
use crate::typesense::client::typesense;
use crate::typesense::documents::anime_document::{AnimeDocument, build_anime_documents};

#[derive(InputObject, Validate)]
#[graphql(name = "CreateAnimeInput")]
pub struct CreateAnimeActionParameters {
    #[validate(length(
        min = 1,
        max = 255,
        message = "The title must be between 1 and 255 characters."
    ))]
    title: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The title_english must be between 1 and 255 characters."
    ))]
    title_english: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The title_native must be between 1 and 255 characters."
    ))]
    title_native: Option<String>,

    season: Option<AnimeSeason>,

    #[validate(range(
        min = 1900,
        max = 2100,
        message = "The year must be between 1900 and 2100."
    ))]
    year: Option<i32>,

    format: Option<AnimeFormat>,

    #[validate(length(
        min = 1,
        max = 1000,
        message = "The synopsis must be between 1 and 1000 characters."
    ))]
    synopsis: Option<String>,
}

pub struct CreateAnimeAction;

impl CreateAnimeAction {
    pub async fn create(
        ctx: &AppContext,
        params: CreateAnimeActionParameters,
        _user: &CurrentUser,
    ) -> Result<anime::Model, AppError> {
        if let Err(errors) = params.validate() {
            return Err(AppError::Validation(errors));
        }

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
