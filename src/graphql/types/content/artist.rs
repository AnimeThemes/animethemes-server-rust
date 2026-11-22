use async_graphql::connection::{Connection, Edge, EmptyFields};
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject};

use crate::entities::content::artist;
use crate::graphql::loaders::content::artist::artist_groups::ArtistGroupsLoader;
use crate::graphql::loaders::content::artist::artist_images::ArtistImagesLoader;
use crate::graphql::loaders::content::artist::artist_members::ArtistMembersLoader;
use crate::graphql::loaders::content::artist::artist_membersongstaffs::ArtistMemberSongStaffsLoader;
use crate::graphql::loaders::content::artist::artist_resources::ArtistResourcesLoader;
use crate::graphql::loaders::content::artist::artist_songstaffs::ArtistSongStaffsLoader;
use crate::graphql::loaders::content::artist::artist_synonyms::ArtistSynonymsLoader;
use crate::graphql::loaders::content::artist::artist_themestaffs::ArtistThemeStaffsLoader;
use crate::graphql::types::content::artist_member::{
    ArtistMemberConnection, ArtistMemberEdge, ArtistMemberEdgeFields,
};
use crate::graphql::types::content::image::Image;
use crate::graphql::types::content::imageable::{
    ImageableConnection, ImageableEdge, ImageableEdgeFields,
};
use crate::graphql::types::content::resource::ExternalResource;
use crate::graphql::types::content::resourceable::{
    ResourceableConnection, ResourceableEdge, ResourceableEdgeFields,
};
use crate::graphql::types::content::song_staff::SongStaff;
use crate::graphql::types::content::synonym::Synonym;
use crate::graphql::types::content::theme_staff::ThemeStaff;

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
    /// The URL for the artist page on the website
    async fn site_url(&self, _ctx: &Context<'_>) -> String {
        format!("https://animethemes.moe/artist/{}", self.slug)
    }

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

    async fn song_staff(&self, ctx: &Context<'_>) -> Result<Vec<SongStaff>> {
        let loader = ctx.data_unchecked::<DataLoader<ArtistSongStaffsLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(SongStaff::from).collect())
    }

    async fn member_song_staff(&self, ctx: &Context<'_>) -> Result<Vec<SongStaff>> {
        let loader = ctx.data_unchecked::<DataLoader<ArtistMemberSongStaffsLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(SongStaff::from).collect())
    }

    async fn theme_staff(&self, ctx: &Context<'_>) -> Result<Vec<ThemeStaff>> {
        let loader = ctx.data_unchecked::<DataLoader<ArtistThemeStaffsLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(ThemeStaff::from).collect())
    }

    #[graphql(deprecation = "Use imagesV2 instead")]
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
        let loader = ctx.data_unchecked::<DataLoader<ArtistImagesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        for model in models {
            connection.edges.push(Edge::with_additional_fields(
                model.id,
                model.clone().into(),
                ImageableEdgeFields {
                    depth: model.depth,
                    created_at: model.created_at,
                    updated_at: model.updated_at,
                },
            ));
        }

        Ok(connection)
    }

    async fn images_v2(&self, ctx: &Context<'_>) -> Result<Vec<Image>> {
        let loader = ctx.data_unchecked::<DataLoader<ArtistImagesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Image::from).collect())
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
        let loader = ctx.data_unchecked::<DataLoader<ArtistResourcesLoader>>();

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
        let loader = ctx.data_unchecked::<DataLoader<ArtistResourcesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(ExternalResource::from).collect())
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
