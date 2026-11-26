use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Error, Result, SimpleObject};

use crate::AppError;
use crate::entities::admin::featuredtheme;
use crate::graphql::loaders::admin::featuredtheme_entry::FeaturedEntryLoader;
use crate::graphql::loaders::admin::featuredtheme_user::FeaturedThemeUserLoader;
use crate::graphql::loaders::admin::featuredtheme_video::FeaturedThemeVideoLoader;
use crate::graphql::types::auth::user::User;
use crate::graphql::types::content::entry::Entry;
use crate::graphql::types::content::video::Video;
use crate::graphql::utils::format_option_datetime;

/// Represents a featured theme on the homepage of the site.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct FeaturedTheme {
    /// The primary key of the resource
    pub id: u64,
    #[graphql(skip)]
    pub start_at: Option<time::OffsetDateTime>,
    #[graphql(skip)]
    pub end_at: Option<time::OffsetDateTime>,
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
    async fn start_at(
        &self,
        #[graphql(default = "%Y-%m-%d %H:%M:%S")] format: String,
    ) -> Result<Option<String>> {
        format_option_datetime(self.start_at.as_ref(), &format)
    }

    /// The end date of the resource
    async fn end_at(
        &self,
        #[graphql(default = "%Y-%m-%d %H:%M:%S")] format: String,
    ) -> Result<Option<String>> {
        format_option_datetime(self.end_at.as_ref(), &format)
    }

    async fn entry(&self, ctx: &Context<'_>) -> Result<Option<Entry>> {
        let Some(entry_id) = self.entry_id else {
            return Ok(None);
        };

        let loader = ctx.data_unchecked::<DataLoader<FeaturedEntryLoader>>();

        let model = loader
            .load_one(entry_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        Ok(Some(model.into()))
    }

    async fn video(&self, ctx: &Context<'_>) -> Result<Option<Video>> {
        let Some(video_id) = self.video_id else {
            return Ok(None);
        };

        let loader = ctx.data_unchecked::<DataLoader<FeaturedThemeVideoLoader>>();

        let model = loader
            .load_one(video_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        Ok(Some(model.into()))
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let Some(user_id) = self.user_id else {
            return Ok(None);
        };

        let loader = ctx.data_unchecked::<DataLoader<FeaturedThemeUserLoader>>();

        Ok(loader.load_one(user_id).await?.map(Into::into))
    }
}

impl From<featuredtheme::Model> for FeaturedTheme {
    fn from(model: featuredtheme::Model) -> Self {
        Self {
            id: model.id,
            start_at: Some(model.start_at),
            end_at: Some(model.end_at),
            entry_id: model.entry_id,
            video_id: model.video_id,
            user_id: model.user_id,
        }
    }
}
