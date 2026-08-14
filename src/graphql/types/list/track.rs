use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::list::track,
    graphql::{
        loaders::list::playlist::{
            track_entry::TrackEntryLoader, track_track::TrackTrackLoader,
            track_video::TrackVideoLoader,
        },
        types::content::{animethemeentry::AnimeThemeEntry, video::Video},
    },
};

/// Represents an entry in a playlist.
///
/// For example, a "/r/anime's Best OPs and EDs of 2022" playlist may contain a track for the ParipiKoumei-OP1.webm video.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct PlaylistTrack {
    /// The primary key of the resource
    #[graphql(name = "id")]
    pub hashid: String,
    #[graphql(skip)]
    pub _id: u64,
    #[graphql(skip)]
    pub entry_id: Option<u64>,
    #[graphql(skip)]
    pub video_id: Option<u64>,
    #[graphql(skip)]
    pub previous_id: Option<u64>,
    #[graphql(skip)]
    pub next_id: Option<u64>,
    /// The position of the playlist track within the playlist
    pub position: i32,
}

#[ComplexObject]
impl PlaylistTrack {
    async fn previous(&self, ctx: &Context<'_>) -> Result<Option<PlaylistTrack>> {
        let Some(previous_id) = self.previous_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<TrackTrackLoader>>()?;

        Ok(loader.load_one(previous_id).await?.map(Into::into))
    }

    async fn next(&self, ctx: &Context<'_>) -> Result<Option<PlaylistTrack>> {
        let Some(next_id) = self.next_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<TrackTrackLoader>>()?;

        Ok(loader.load_one(next_id).await?.map(Into::into))
    }

    async fn animethemeentry(&self, ctx: &Context<'_>) -> Result<Option<AnimeThemeEntry>> {
        let Some(entry_id) = self.entry_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<TrackEntryLoader>>()?;

        Ok(loader.load_one(entry_id).await?.map(Into::into))
    }

    async fn video(&self, ctx: &Context<'_>) -> Result<Option<Video>> {
        let Some(video_id) = self.video_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<TrackVideoLoader>>()?;

        Ok(loader.load_one(video_id).await?.map(Into::into))
    }
}

impl From<track::Model> for PlaylistTrack {
    fn from(model: track::Model) -> Self {
        Self {
            hashid: model.hashid.unwrap(),
            _id: model.id,
            entry_id: model.entry_id,
            video_id: model.video_id,
            previous_id: model.previous_id,
            next_id: model.next_id,
            position: model.position,
        }
    }
}
