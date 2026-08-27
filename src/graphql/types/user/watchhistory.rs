use crate::{AppError, entities::user::watchhistory};
use async_graphql::{Context, Error, Object, Result, dataloader::DataLoader};

use crate::graphql::{
    loaders::user::watchhistory::{
        watchhistory_entry::WatchHistoryEntryLoader, watchhistory_video::WatchHistoryVideoLoader,
    },
    types::content::{animethemeentry::AnimeThemeEntry, video::Video},
};

/// Represents the watch history of the authenticated user.
pub struct WatchHistory {
    pub entry_id: u64,
    pub video_id: u64,
}

#[Object]
impl WatchHistory {
    async fn animethemeentry(&self, ctx: &Context<'_>) -> Result<AnimeThemeEntry> {
        let loader = ctx.data_unchecked::<DataLoader<WatchHistoryEntryLoader>>();

        Ok(loader
            .load_one(self.entry_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?
            .into())
    }

    async fn video(&self, ctx: &Context<'_>) -> Result<Video> {
        let loader = ctx.data_unchecked::<DataLoader<WatchHistoryVideoLoader>>();

        Ok(loader
            .load_one(self.video_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?
            .into())
    }
}

impl From<watchhistory::Model> for WatchHistory {
    fn from(model: watchhistory::Model) -> Self {
        Self {
            entry_id: model.entry_id,
            video_id: model.video_id,
        }
    }
}
