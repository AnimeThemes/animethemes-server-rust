use async_graphql::connection::{Connection, Edge, EmptyFields};
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject};

use crate::entities::content::{imageable, studio};
use crate::graphql::loaders::content::imageable::{ImageableKey, ImageableLoader};
use crate::graphql::loaders::content::studio::studio_anime::StudioAnimeLoader;
use crate::graphql::loaders::content::studio::studio_resources::StudioResourcesLoader;
use crate::graphql::types::content::anime::Anime;
use crate::graphql::types::content::image::Image;
use crate::graphql::types::content::imageable::{
    ImageableConnection, ImageableEdge, ImageableEdgeFields,
};
use crate::graphql::types::content::resource::ExternalResource;
use crate::graphql::types::content::resourceable::{
    ResourceableConnection, ResourceableEdge, ResourceableEdgeFields,
};
use crate::graphql::types::content::studio_anime::{
    StudioAnimeConnection, StudioAnimeEdge, StudioAnimeEdgeFields,
};

/// Represents a company that produces anime.
///
/// For example, Shaft is the studio that produced the anime Bakemonogatari.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Studio {
    /// The primary key of the resource
    pub id: u64,
    /// The primary title of the Studio
    pub name: String,
    /// The URL slug & route key of the resource
    pub slug: String,
}

#[ComplexObject]
impl Studio {
    /// The URL for the studio page on the website
    async fn site_url(&self, _ctx: &Context<'_>) -> String {
        format!("https://animethemes.moe/studio/{}", self.slug)
    }

    async fn anime(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            Anime,
            EmptyFields,
            StudioAnimeEdgeFields,
            StudioAnimeConnection,
            StudioAnimeEdge,
        >,
    > {
        let loader = ctx.data_unchecked::<DataLoader<StudioAnimeLoader>>();

        let rows = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, anime) in rows {
            connection.edges.push(Edge::with_additional_fields(
                anime.id,
                anime.into(),
                StudioAnimeEdgeFields {
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
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
        let loader = ctx.data_unchecked::<DataLoader<ImageableLoader>>();

        let rows = loader
            .load_one(ImageableKey {
                id: self.id,
                imageable_type: imageable::ImageableType::Studio,
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

    #[graphql(deprecation = "Use resourcesV2 instead")]
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
        let loader = ctx.data_unchecked::<DataLoader<StudioResourcesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for model in models {
            connection.edges.push(Edge::with_additional_fields(
                model.id,
                model.clone().into(),
                ResourceableEdgeFields {
                    r#as: model.r#as,
                    created_at: model.created_at,
                    updated_at: model.updated_at,
                },
            ));
        }

        Ok(connection)
    }

    async fn resources_v2(&self, ctx: &Context<'_>) -> Result<Vec<ExternalResource>> {
        let loader = ctx.data_unchecked::<DataLoader<StudioResourcesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(ExternalResource::from).collect())
    }
}

impl From<studio::Model> for Studio {
    fn from(model: studio::Model) -> Self {
        Self {
            id: model.id,
            slug: model.slug,
            name: model.name,
        }
    }
}
