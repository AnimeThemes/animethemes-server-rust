use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};
use chrono::{DateTime, Utc};

use crate::{
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
        utils::format_datetime,
    },
};

/// Represents a video to be featured on the homepage of the site for a specified amount of time.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct FeaturedTheme {
    /// The primary key of the resource
    pub id: u64,
    #[graphql(skip)]
    pub start_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    pub end_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    pub entry_id: Option<u64>,
    #[graphql(skip)]
    pub video_id: Option<u64>,
    #[graphql(skip)]
    pub user_id: Option<u64>,
}

#[ComplexObject]
impl FeaturedTheme {
    /// The start date of the resource
    async fn start_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.start_at.as_ref(), &format)
    }

    /// The end date of the resource
    async fn end_at(&self, #[graphql(default = "%+")] format: String) -> Option<String> {
        format_datetime(self.end_at.as_ref(), &format)
    }

    async fn animethemeentry(&self, ctx: &Context<'_>) -> Result<Option<AnimeThemeEntry>> {
        let Some(entry_id) = self.entry_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<FeaturedThemeEntryLoader>>()?;

        Ok(loader.load_one(entry_id).await?.map(Into::into))
    }

    async fn video(&self, ctx: &Context<'_>) -> Result<Option<Video>> {
        let Some(video_id) = self.video_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<FeaturedThemeVideoLoader>>()?;

        Ok(loader.load_one(video_id).await?.map(Into::into))
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let Some(user_id) = self.user_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<FeaturedThemeUserLoader>>()?;

        Ok(loader.load_one(user_id).await?.map(Into::into))
    }
}

impl From<featuredtheme::Model> for FeaturedTheme {
    fn from(model: featuredtheme::Model) -> Self {
        Self {
            id: model.id,
            start_at: model.start_at,
            end_at: model.end_at,
            entry_id: model.entry_id,
            video_id: model.video_id,
            user_id: model.user_id,
        }
    }
}
