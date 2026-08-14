use async_graphql::{
    ComplexObject, Context, Result, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
    dataloader::DataLoader,
};

use crate::{
    entities::content::series,
    graphql::{
        loaders::content::series::series_anime::SeriesAnimeLoader,
        types::content::{
            anime::Anime,
            anime_series::{SeriesAnimeConnection, SeriesAnimeEdge, SeriesAnimeEdgeFields},
        },
    },
};

#[derive(SimpleObject)]
pub struct SeriesTitle {
    romaji: String,
}

impl From<&series::Model> for SeriesTitle {
    fn from(model: &series::Model) -> Self {
        Self {
            romaji: model.title.clone(),
        }
    }
}

/// Represents a collection of related anime.
///
/// For example, the Monogatari series is the collection of the Bakemonogatari anime and its related productions.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Series {
    /// The primary key of the resource
    pub id: u64,
    /// The primary title of the series
    pub title: SeriesTitle,
    /// The URL slug & route key of the resource
    pub slug: String,
}

#[ComplexObject]
impl Series {
    async fn anime(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            Anime,
            EmptyFields,
            SeriesAnimeEdgeFields,
            SeriesAnimeConnection,
            SeriesAnimeEdge,
        >,
    > {
        let loader = ctx.data::<DataLoader<SeriesAnimeLoader>>()?;

        let rows = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, series) in rows {
            connection.edges.push(Edge::with_additional_fields(
                series.id,
                series.into(),
                SeriesAnimeEdgeFields {
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }
}

impl From<series::Model> for Series {
    fn from(model: series::Model) -> Self {
        let title = SeriesTitle::from(&model);
        Self {
            id: model.id,
            slug: model.slug,
            title,
        }
    }
}
