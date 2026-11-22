use async_graphql::connection::{Connection, Edge, EmptyFields};
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject};

use crate::entities::content::song;
use crate::graphql::loaders::content::song::song_resources::SongResourcesLoader;
use crate::graphql::loaders::content::song::song_staffs::SongStaffsLoader;
use crate::graphql::loaders::content::song::song_themes::SongThemesLoader;
use crate::graphql::types::content::resource::ExternalResource;
use crate::graphql::types::content::resourceable::{
    ResourceableConnection, ResourceableEdge, ResourceableEdgeFields,
};
use crate::graphql::types::content::song_staff::SongStaff;
use crate::graphql::types::content::theme::Theme;

#[derive(SimpleObject)]
pub struct SongTitle {
    /// The romaji title of the composition
    romaji: Option<String>,
    /// The native title of the composition
    native: Option<String>,
}

impl From<&song::Model> for SongTitle {
    fn from(model: &song::Model) -> Self {
        Self {
            romaji: model.title.clone(),
            native: model.title_native.clone(),
        }
    }
}

/// Represents the composition that accompanies an Theme.
///
/// For example, Staple Stable is the song for the Bakemonogatari OP1 Theme.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Song {
    /// The primary key of the resource
    pub id: u64,
    /// The title of the composition
    pub title: SongTitle,
}

#[ComplexObject]
impl Song {
    async fn themes(&self, ctx: &Context<'_>) -> Result<Vec<Theme>> {
        let loader = ctx.data_unchecked::<DataLoader<SongThemesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Theme::from).collect())
    }

    async fn staff(&self, ctx: &Context<'_>) -> Result<Vec<SongStaff>> {
        let loader = ctx.data_unchecked::<DataLoader<SongStaffsLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(SongStaff::from).collect())
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
        let loader = ctx.data_unchecked::<DataLoader<SongResourcesLoader>>();

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
        let loader = ctx.data_unchecked::<DataLoader<SongResourcesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(ExternalResource::from).collect())
    }
}

impl From<song::Model> for Song {
    fn from(model: song::Model) -> Self {
        let title = SongTitle::from(&model);
        Self {
            id: model.id,
            title,
        }
    }
}
