use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Error, Object, Result};

use crate::AppError;
use crate::entities::user::watchhistory;
use crate::graphql::loaders::user::watchhistory::watchhistory_entry::WatchHistoryEntryLoader;
use crate::graphql::loaders::user::watchhistory::watchhistory_video::WatchHistoryVideoLoader;
use crate::graphql::types::content::entry::Entry;
use crate::graphql::types::content::video::Video;

/// Represents the watch history of the authenticated user.
pub struct WatchHistory {
    pub entry_id: u64,
    pub video_id: u64,
}

#[Object]
impl WatchHistory {
    async fn entry(&self, ctx: &Context<'_>) -> Result<Entry> {
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
