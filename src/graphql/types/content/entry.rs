use async_graphql::{
    ComplexObject, Context, Result, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
    dataloader::DataLoader,
};

use crate::{
    entities::content::entry,
    graphql::{
        loaders::content::entry::{entry_theme::EntryThemeLoader, entry_videos::EntryVideosLoader},
        types::content::{
            video::Video,
            {
                entry_video::{EntryVideoConnection, EntryVideoEdge, EntryVideoEdgeFields},
                theme::Theme,
            },
        },
    },
};

/// Represents a version of a theme.
///
/// For example, the ED theme of the Bakemonogatari anime has three theme entries to represent three versions.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Entry {
    /// The primary key of the resource
    pub id: u64,
    #[graphql(skip)]
    pub theme_id: u64,
    /// The episodes that the theme is used for
    pub episodes: Option<String>,
    /// The number of favorites recorded for the resource
    pub favorites_count: i32,
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
impl Entry {
    async fn theme(&self, ctx: &Context<'_>) -> Result<Theme> {
        let loader = ctx.data_unchecked::<DataLoader<EntryThemeLoader>>();

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
            EntryVideoEdgeFields,
            EntryVideoConnection,
            EntryVideoEdge,
        >,
    > {
        let loader = ctx.data_unchecked::<DataLoader<EntryVideosLoader>>();

        let rows = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, video) in rows {
            connection.edges.push(Edge::with_additional_fields(
                video.id,
                video.into(),
                EntryVideoEdgeFields {
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }
}

impl From<entry::Model> for Entry {
    fn from(model: entry::Model) -> Self {
        Self {
            id: model.id,
            theme_id: model.theme_id,
            episodes: model.episodes,
            favorites_count: model.favorites_count,
            notes: model.notes,
            nsfw: model.nsfw,
            spoiler: model.spoiler,
            tracks_count: model.tracks_count,
            version: model.version,
        }
    }
}
