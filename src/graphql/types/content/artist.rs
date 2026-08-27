use async_graphql::{
    ComplexObject, Context, Result, SimpleObject,
    connection::{Connection, Edge, EmptyFields},
    dataloader::DataLoader,
};

use crate::{
    entities::content::artist,
    graphql::{
        loaders::content::{
            artist::{
                artist_groups::ArtistGroupsLoader,
                artist_memberperformances::ArtistMemberPerformancesLoader,
                artist_members::ArtistMembersLoader, artist_performances::ArtistPerformancesLoader,
                artist_synonyms::ArtistSynonymsLoader,
            },
            imageable::{ImageableKey, ImageableLoader},
            resourceable::{ResourceableKey, ResourceableLoader},
        },
        types::content::{
            artist_member::{ArtistMemberConnection, ArtistMemberEdge, ArtistMemberEdgeFields},
            externalresource::ExternalResource,
            image::Image,
            imageable::{ImageableConnection, ImageableEdge, ImageableEdgeFields},
            performance::Performance,
            resourceable::{ResourceableConnection, ResourceableEdge, ResourceableEdgeFields},
            synonym::Synonym,
        },
    },
};

#[derive(SimpleObject)]
pub struct ArtistName {
    /// The stylized name of the artist
    main: String,
    /// The native name of the artist
    native: Option<String>,
}

impl From<&artist::Model> for ArtistName {
    fn from(model: &artist::Model) -> Self {
        Self {
            main: model.name.clone(),
            native: model.name_native.clone(),
        }
    }
}

/// Represents a musical performer of anime sequences.
///
/// For example, Chiwa Saitou is the musical performer of the Bakemonogatari OP1 theme, among many others.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Artist {
    /// The primary key of the resource
    pub id: u64,
    /// The primary title of the artist
    pub name: ArtistName,
    /// The URL slug & route key of the resource
    pub slug: String,
    /// The brief information of the resource
    pub information: Option<String>,
}

#[ComplexObject]
impl Artist {
    async fn synonyms(&self, ctx: &Context<'_>) -> Result<Vec<Synonym>> {
        let loader = ctx.data_unchecked::<DataLoader<ArtistSynonymsLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Synonym::from).collect())
    }

    async fn members(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            Artist,
            EmptyFields,
            ArtistMemberEdgeFields,
            ArtistMemberConnection,
            ArtistMemberEdge,
        >,
    > {
        let loader = ctx.data_unchecked::<DataLoader<ArtistMembersLoader>>();

        let rows = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, member) in rows {
            connection.edges.push(Edge::with_additional_fields(
                member.id,
                member.into(),
                ArtistMemberEdgeFields {
                    alias: pivot.alias,
                    r#as: pivot.r#as,
                    notes: pivot.notes,
                    relevance: pivot.relevance,
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }

    async fn groups(
        &self,
        ctx: &Context<'_>,
    ) -> Result<
        Connection<
            u64,
            Artist,
            EmptyFields,
            ArtistMemberEdgeFields,
            ArtistMemberConnection,
            ArtistMemberEdge,
        >,
    > {
        let loader = ctx.data_unchecked::<DataLoader<ArtistGroupsLoader>>();

        let rows = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for (pivot, group) in rows {
            connection.edges.push(Edge::with_additional_fields(
                group.id,
                group.into(),
                ArtistMemberEdgeFields {
                    alias: pivot.alias,
                    r#as: pivot.r#as,
                    notes: pivot.notes,
                    relevance: pivot.relevance,
                    created_at: pivot.created_at,
                    updated_at: pivot.updated_at,
                },
            ));
        }

        Ok(connection)
    }

    async fn performances(&self, ctx: &Context<'_>) -> Result<Vec<Performance>> {
        let loader = ctx.data_unchecked::<DataLoader<ArtistPerformancesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Performance::from).collect())
    }

    async fn member_performances(&self, ctx: &Context<'_>) -> Result<Vec<Performance>> {
        let loader = ctx.data_unchecked::<DataLoader<ArtistMemberPerformancesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Performance::from).collect())
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
                imageable_type: "artist".to_string(),
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
        let loader = ctx.data_unchecked::<DataLoader<ResourceableLoader>>();

        let rows = loader
            .load_one(ResourceableKey {
                id: self.id,
                resourceable_type: "artist".to_string(),
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
}

impl From<artist::Model> for Artist {
    fn from(model: artist::Model) -> Self {
        let name = ArtistName::from(&model);
        Self {
            id: model.id,
            slug: model.slug,
            name,
            information: model.information,
        }
    }
}
