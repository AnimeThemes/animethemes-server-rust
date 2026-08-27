use async_graphql::{ComplexObject, Context, Error, Result, SimpleObject, dataloader::DataLoader};
use chrono::{DateTime, Utc};

use crate::{
    AppError,
    entities::admin::featuredtheme,
    graphql::{
        loaders::admin::{
            featuredtheme_entry::FeaturedThemeEntryLoader,
            featuredtheme_user::FeaturedThemeUserLoader,
            featuredtheme_video::FeaturedThemeVideoLoader,
        },
        types::{
            auth::user::User,
            content::{animethemeentry::AnimeThemeEntry, video::Video},
        },
        utils::format_option_datetime,
    },
};

/// Represents the current featured theme on the homepage of the site.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct CurrentFeaturedTheme {
    /// The primary key of the resource
    pub id: u64,
    #[graphql(skip)]
    pub start_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    pub end_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    pub entry_id: u64,
    #[graphql(skip)]
    pub video_id: u64,
    #[graphql(skip)]
    pub user_id: Option<u64>,
}

#[ComplexObject]
impl CurrentFeaturedTheme {
    /// The start date of the resource
    async fn start_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_option_datetime(self.start_at.as_ref(), &format)
    }

    /// The end date of the resource
    async fn end_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_option_datetime(self.end_at.as_ref(), &format)
    }

    async fn animethemeentry(&self, ctx: &Context<'_>) -> Result<AnimeThemeEntry> {
        let loader = ctx.data_unchecked::<DataLoader<FeaturedThemeEntryLoader>>();

        let model = loader
            .load_one(self.entry_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        Ok(model.into())
    }

    async fn video(&self, ctx: &Context<'_>) -> Result<Video> {
        let loader = ctx.data_unchecked::<DataLoader<FeaturedThemeVideoLoader>>();

        let model = loader
            .load_one(self.video_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        Ok(model.into())
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let Some(user_id) = self.user_id else {
            return Ok(None);
        };

        let loader = ctx.data_unchecked::<DataLoader<FeaturedThemeUserLoader>>();

        Ok(loader.load_one(user_id).await?.map(Into::into))
    }
}

impl From<featuredtheme::Model> for CurrentFeaturedTheme {
    fn from(model: featuredtheme::Model) -> Self {
        Self {
            id: model.id,
            start_at: model.start_at,
            end_at: model.end_at,
            entry_id: model
                .entry_id
                .expect("entry_id is required for the current featured theme"),
            video_id: model
                .video_id
                .expect("video_id is required for the current featured theme"),
            user_id: model.user_id,
        }
    }
}
