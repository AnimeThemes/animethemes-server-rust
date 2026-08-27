use async_graphql::{
    ComplexObject, Context, Result, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
    dataloader::DataLoader,
};

use crate::{
    entities::content::animethemeentry,
    graphql::{
        loaders::content::animethemeentry::{
            animethemeentry_theme::AnimeThemeEntryThemeLoader,
            animethemeentry_videos::AnimeThemeEntryVideosLoader,
        },
        types::content::{
            video::Video,
            {
                animetheme::AnimeTheme,
                animethemeentry_video::{
                    AnimeThemeEntryVideoConnection, AnimeThemeEntryVideoEdge,
                    AnimeThemeEntryVideoEdgeFields,
                },
            },
        },
    },
};

/// Represents a version of an anime theme.
///
/// For example, the ED theme of the Bakemonogatari anime has three anime theme entries to represent three versions.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct AnimeThemeEntry {
    /// The primary key of the resource
    pub id: u64,
    #[graphql(skip)]
    pub theme_id: u64,
    /// The episodes that the theme is used for
    pub episodes: Option<String>,
    /// The number of likes recorded for the resource
    pub likes_count: i32,
    /// Any additional information for this sequence
    pub notes: Option<String>,
    /// Is not safe for work content included?
    pub nsfw: bool,
    /// Is content included that may spoil the viewer?
    pub spoiler: bool,
    /// The number of tracks belonging to the resource
    pub tracks_count: i32,
    /// The version number of the theme
    pub version: i32,
}

#[ComplexObject]
impl AnimeThemeEntry {
    async fn animetheme(&self, ctx: &Context<'_>) -> Result<AnimeTheme> {
        let loader = ctx.data_unchecked::<DataLoader<AnimeThemeEntryThemeLoader>>();

        let theme = loader
            .load_one(self.theme_id)
            .await?
            .ok_or("Theme not found")?;

        Ok(theme.into())
    }

    async fn videos(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            Video,
            EmptyFields,
            AnimeThemeEntryVideoEdgeFields,
            AnimeThemeEntryVideoConnection,
            AnimeThemeEntryVideoEdge,
        >,
    > {
        let loader = ctx.data_unchecked::<DataLoader<AnimeThemeEntryVideosLoader>>();

        let rows = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, video) in rows {
            connection.edges.push(Edge::with_additional_fields(
                video.id,
                video.into(),
                AnimeThemeEntryVideoEdgeFields {
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }
}

impl From<animethemeentry::Model> for AnimeThemeEntry {
    fn from(model: animethemeentry::Model) -> Self {
        Self {
            id: model.id,
            theme_id: model.theme_id,
            episodes: model.episodes,
            likes_count: model.likes_count,
            notes: model.notes,
            nsfw: model.nsfw,
            spoiler: model.spoiler,
            tracks_count: model.tracks_count,
            version: model.version,
        }
    }
}
