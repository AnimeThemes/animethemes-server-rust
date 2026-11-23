use anyhow::Context;
use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use validator::{Validate, ValidationError};

use crate::AppError;
use crate::entities::content::anime;
use crate::enums::content::animeformat::AnimeFormat;
use crate::enums::content::animeseason::AnimeSeason;
use crate::middlewares::current_user::CurrentUser;
use crate::typesense::client::typesense;
use crate::typesense::documents::anime_document::{AnimeDocument, build_anime_documents};

#[derive(InputObject, Validate)]
#[graphql(name = "SaveAnimeInput")]
pub struct SaveAnimeInput {
    #[validate(nested)]
    title: Option<AnimeTitleInput>,

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

#[derive(InputObject, Validate)]
#[graphql(name = "AnimeTitleInput")]
pub struct AnimeTitleInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "The romaji title must be between 1 and 255 characters."
    ))]
    pub romaji: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The english title must be between 1 and 255 characters."
    ))]
    pub english: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "The native title must be between 1 and 255 characters."
    ))]
    pub native: Option<String>,
}

pub struct SaveAnimeAction;

impl SaveAnimeAction {
    pub async fn save(
        ctx: &AppContext,
        id: Option<u64>,
        params: SaveAnimeInput,
        _user: &CurrentUser,
    ) -> Result<anime::Model, AppError> {
        let mut errors = params.validate().err().unwrap_or_default();

        if id.is_none()
            && params
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

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        let mut anime: anime::ActiveModel = Default::default();

        if let Some(id) = id {
            anime.id = Set(id);
        }

        if let Some(title) = params.title {
            if let Some(title_romaji) = title.romaji {
                anime.title = Set(title_romaji);
            }

            anime.title_english = Set(title.english);
            anime.title_native = Set(title.native);
        }

        if let Some(season) = params.season {
            anime.season = Set(Some(season));
        }

        if let Some(year) = params.year {
            anime.year = Set(Some(year));
        }

        if let Some(format) = params.format {
            anime.format = Set(Some(format));
        }

        if let Some(synopsis) = params.synopsis {
            anime.synopsis = Set(Some(synopsis));
        }

        let anime = if id.is_some() {
            anime.update(&ctx.db).await?
        } else {
            anime.insert(&ctx.db).await?
        };

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
