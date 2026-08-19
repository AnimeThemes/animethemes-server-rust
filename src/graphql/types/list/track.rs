use async_graphql::{ComplexObject, Context, Error, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::list::track,
    graphql::{
        loaders::list::playlist::{
            track_entry::TrackEntryLoader, track_playlist::TrackPlaylistLoader,
            track_video::TrackVideoLoader,
        },
        types::{
            content::{animethemeentry::AnimeThemeEntry, video::Video},
            list::playlist::Playlist,
        },
    },
    policies::AppError,
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
    pub playlist_id: u64,
    #[graphql(skip)]
    pub entry_id: u64,
    #[graphql(skip)]
    pub video_id: u64,
    /// The position of the playlist track within the playlist
    pub position: i32,
}

#[ComplexObject]
impl PlaylistTrack {
    async fn playlist(&self, ctx: &Context<'_>) -> Result<Playlist> {
        let loader = ctx.data::<DataLoader<TrackPlaylistLoader>>()?;

        Ok(loader
            .load_one(self.playlist_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?
            .into())
    }

    async fn animethemeentry(&self, ctx: &Context<'_>) -> Result<AnimeThemeEntry> {
        let loader = ctx.data::<DataLoader<TrackEntryLoader>>()?;

        Ok(loader
            .load_one(self.entry_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?
            .into())
    }

    async fn video(&self, ctx: &Context<'_>) -> Result<Video> {
        let loader = ctx.data::<DataLoader<TrackVideoLoader>>()?;

        Ok(loader
            .load_one(self.video_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?
            .into())
    }
}

impl From<track::Model> for PlaylistTrack {
    fn from(model: track::Model) -> Self {
        Self {
            hashid: model.hashid.unwrap(),
            _id: model.id,
            playlist_id: model.playlist_id,
            entry_id: model.entry_id.expect("entry_id is required for track"),
            video_id: model.video_id.expect("video_id is required for track"),
            position: model.position,
        }
    }
}
