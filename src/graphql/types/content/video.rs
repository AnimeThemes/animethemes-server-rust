use animethemes_graphql_rust::enums::LocalizedEnum;
use async_graphql::{
    ComplexObject, Context, Result, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
    dataloader::DataLoader,
};

use crate::{
    entities::content::video::{self},
    graphql::{
        enums::content::{videooverlap::VideoOverlap, videosource::VideoSource},
        loaders::content::video::{
            video_animethemeentries::VideoAnimeThemeEntriesLoader, video_audio::VideoAudioLoader,
            video_script::VideoScriptLoader, video_tracks::VideoTracksLoader,
        },
        types::{
            content::{
                animethemeentry::AnimeThemeEntry,
                animethemeentry_video::{
                    VideoAnimeThemeEntryConnection, VideoAnimeThemeEntryEdge,
                    VideoAnimeThemeEntryEdgeFields,
                },
                audio::Audio,
                videoscript::VideoScript,
            },
            list::track::PlaylistTrack,
        },
    },
};

/// Represents a WebM of an anime theme.
///
/// For example, the video Bakemonogatari-OP1.webm represents the WebM of the Bakemonogatari OP1 theme.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Video {
    /// The primary key of the resource
    pub id: u64,
    #[graphql(skip)]
    pub audio_id: Option<u64>,
    /// The basename of the file in storage
    pub basename: String,
    /// The filename of the file in storage
    pub filename: String,
    /// The URL to stream the file from storage
    pub link: String,
    /// Does the video include subtitles of song lyrics?
    pub lyrics: bool,
    /// The media type of the file in storage
    pub mimetype: String,
    /// Is the video creditless?
    pub nc: bool,
    /// The degree to which the sequence and episode content overlap
    pub overlap: VideoOverlap,
    /// The formatted string value of the overlap field
    pub overlap_localized: String,
    /// The path of the file in storage
    pub path: String,
    /// The frame height of the file in storage
    pub resolution: Option<i32>,
    /// The size of the file in storage in Bytes
    pub size: Option<i32>,
    /// Where did this video come from?
    pub source: Option<VideoSource>,
    /// The formatted string value of the source field
    pub source_localized: Option<String>,
    /// Does the video include subtitles of dialogue?
    pub subbed: bool,
    /// The attributes used to distinguish the file within the context of a theme
    pub tags: String,
    /// Is the video an uncensored version of a censored sequence?
    pub uncen: bool,
}

#[ComplexObject]
impl Video {
    async fn animethemeentries(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            AnimeThemeEntry,
            EmptyFields,
            VideoAnimeThemeEntryEdgeFields,
            VideoAnimeThemeEntryConnection,
            VideoAnimeThemeEntryEdge,
        >,
    > {
        let loader = ctx.data::<DataLoader<VideoAnimeThemeEntriesLoader>>()?;

        let rows = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, model) in rows {
            connection.edges.push(Edge::with_additional_fields(
                model.id,
                model.into(),
                VideoAnimeThemeEntryEdgeFields {
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }

    async fn audio(&self, ctx: &Context<'_>) -> Result<Option<Audio>> {
        let Some(audio_id) = self.audio_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<VideoAudioLoader>>()?;

        Ok(loader.load_one(audio_id).await?.map(Into::into))
    }

    async fn videoscript(&self, ctx: &Context<'_>) -> Result<Option<VideoScript>> {
        let loader = ctx.data::<DataLoader<VideoScriptLoader>>()?;

        let script = loader.load_one(self.id).await?;

        Ok(script.map(Into::into))
    }

    async fn tracks(&self, ctx: &Context<'_>) -> Result<Vec<PlaylistTrack>> {
        let loader = ctx.data::<DataLoader<VideoTracksLoader>>()?;

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(PlaylistTrack::from).collect())
    }
}

impl From<video::Model> for Video {
    fn from(model: video::Model) -> Self {
        Self {
            id: model.id,
            audio_id: model.audio_id,
            basename: model.basename.clone(),
            filename: model.filename.clone(),
            link: model.link(),
            lyrics: model.lyrics,
            mimetype: model.mimetype.clone(),
            nc: model.nc,
            overlap: model.overlap.into(),
            overlap_localized: model.overlap.localize().to_string(),
            path: model.path.clone(),
            resolution: model.resolution,
            size: model.size,
            source: model.source.map(Into::into),
            source_localized: model.source.map(|s| s.localize().to_string()),
            subbed: model.subbed,
            tags: model.tags(),
            uncen: model.uncen,
        }
    }
}
