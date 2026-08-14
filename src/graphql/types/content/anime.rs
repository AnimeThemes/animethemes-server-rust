use animethemes_server_rust::enums::LocalizedEnum;
use async_graphql::{
    ComplexObject, Context, Result, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
    dataloader::DataLoader,
};

use crate::{
    entities::content::anime::{self},
    graphql::{
        enums::content::{animeformat::AnimeFormat, animeseason::AnimeSeason},
        loaders::content::{
            anime::{
                anime_series::AnimeSeriesLoader, anime_studios::AnimeStudiosLoader,
                anime_synonyms::AnimeSynonymsLoader, anime_themes::AnimeThemesLoader,
            },
            imageable::{ImageableKey, ImageableLoader},
            resourceable::{ResourceableKey, ResourceableLoader},
        },
        types::content::{
            anime_series::{AnimeSeriesConnection, AnimeSeriesEdge, AnimeSeriesEdgeFields},
            anime_studios::{AnimeStudioConnection, AnimeStudioEdge, AnimeStudioEdgeFields},
            animetheme::AnimeTheme,
            externalresource::ExternalResource,
            image::Image,
            imageable::{ImageableConnection, ImageableEdge, ImageableEdgeFields},
            resourceable::{ResourceableConnection, ResourceableEdge, ResourceableEdgeFields},
            series::Series,
            studio::Studio,
            synonym::Synonym,
        },
    },
};

#[derive(SimpleObject)]
pub struct AnimeTitle {
    romaji: String,
    english: Option<String>,
    native: Option<String>,
}

impl From<&anime::Model> for AnimeTitle {
    fn from(model: &anime::Model) -> Self {
        Self {
            romaji: model.title.clone(),
            english: model.title_english.clone(),
            native: model.title_native.clone(),
        }
    }
}

/// Represents a production with at least one opening or ending sequence.
///
/// For example, Bakemonogatari is an anime production with five opening sequences and one ending sequence.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Anime {
    /// The primary key of the resource
    pub id: u64,
    /// The primary title of the anime
    pub title: AnimeTitle,
    /// The format of the anime
    pub format: Option<AnimeFormat>,
    /// The localized string value of the format field
    pub format_localized: Option<String>,
    /// The premiere season of the anime
    pub season: Option<AnimeSeason>,
    /// The localized string value of the season field
    pub season_localized: Option<String>,
    /// The URL slug & route key of the resource
    pub slug: String,
    /// The brief summary of the anime
    pub synopsis: Option<String>,
    /// The premiere season year of the anime
    pub year: Option<i32>,
}

#[ComplexObject]
impl Anime {
    async fn synonyms(&self, ctx: &Context<'_>) -> Result<Vec<Synonym>> {
        let loader = ctx.data::<DataLoader<AnimeSynonymsLoader>>()?;

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Synonym::from).collect())
    }

    async fn animethemes(&self, ctx: &Context<'_>) -> Result<Vec<AnimeTheme>> {
        let loader = ctx.data::<DataLoader<AnimeThemesLoader>>()?;

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(AnimeTheme::from).collect())
    }

    async fn images(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            Image,
            EmptyFields,
            ImageableEdgeFields,
            ImageableConnection,
            ImageableEdge,
        >,
    > {
        let loader = ctx.data::<DataLoader<ImageableLoader>>()?;

        let rows = loader
            .load_one(ImageableKey {
                id: self.id,
                imageable_type: "anime".to_string(),
            })
            .await?
            .unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, image) in rows {
            connection.edges.push(Edge::with_additional_fields(
                image.id,
                image.into(),
                ImageableEdgeFields {
                    depth: pivot.depth,
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }

    async fn resources(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            ExternalResource,
            EmptyFields,
            ResourceableEdgeFields,
            ResourceableConnection,
            ResourceableEdge,
        >,
    > {
        let loader = ctx.data::<DataLoader<ResourceableLoader>>()?;

        let rows = loader
            .load_one(ResourceableKey {
                id: self.id,
                resourceable_type: "anime".to_string(),
            })
            .await?
            .unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, resource) in rows {
            connection.edges.push(Edge::with_additional_fields(
                resource.id,
                resource.into(),
                ResourceableEdgeFields {
                    r#as: pivot.r#as,
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }

    async fn series(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            Series,
            EmptyFields,
            AnimeSeriesEdgeFields,
            AnimeSeriesConnection,
            AnimeSeriesEdge,
        >,
    > {
        let loader = ctx.data::<DataLoader<AnimeSeriesLoader>>()?;

        let rows = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, series) in rows {
            connection.edges.push(Edge::with_additional_fields(
                series.id,
                series.into(),
                AnimeSeriesEdgeFields {
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }

    async fn studios(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            Studio,
            EmptyFields,
            AnimeStudioEdgeFields,
            AnimeStudioConnection,
            AnimeStudioEdge,
        >,
    > {
        let loader = ctx.data::<DataLoader<AnimeStudiosLoader>>()?;

        let rows = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, studio) in rows {
            connection.edges.push(Edge::with_additional_fields(
                studio.id,
                studio.into(),
                AnimeStudioEdgeFields {
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }
}

impl From<anime::Model> for Anime {
    fn from(model: anime::Model) -> Self {
        let title = AnimeTitle::from(&model);
        Self {
            id: model.id,
            format: model.format.map(Into::into),
            format_localized: model.format.map(|f| f.localize().to_string()),
            season: model.season.map(Into::into),
            season_localized: model.season.map(|s| s.localize().to_string()),
            slug: model.slug,
            synopsis: model.synopsis,
            title,
            year: model.year,
        }
    }
}
