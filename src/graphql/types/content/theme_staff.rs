use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::content::theme_staff,
    graphql::{
        loaders::content::themestaff::{
            themestaff_artist::ThemeStaffArtistLoader, themestaff_theme::ThemeStaffThemeLoader,
        },
        types::content::{artist::Artist, theme::Theme},
    },
};

/// Represents the link between a theme and an artist.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ThemeStaff {
    /// The primary key of the resource
    pub id: u64,
    #[graphql(skip)]
    pub artist_id: u64,
    #[graphql(skip)]
    pub theme_id: u64,
    /// The alias the artist is using for this staff
    pub alias: Option<String>,
    /// Used to determine the relevance order of artists in staffs
    pub relevance: i32,
    /// The role the artist is performing
    pub role: String,
}

#[ComplexObject]
impl ThemeStaff {
    async fn artist(&self, ctx: &Context<'_>) -> Result<Artist> {
        let loader = ctx.data_unchecked::<DataLoader<ThemeStaffArtistLoader>>();

        let model = loader
            .load_one(self.artist_id)
            .await?
            .ok_or("Artist not found")?;

        Ok(model.into())
    }

    async fn theme(&self, ctx: &Context<'_>) -> Result<Theme> {
        let loader = ctx.data_unchecked::<DataLoader<ThemeStaffThemeLoader>>();

        let model = loader
            .load_one(self.theme_id)
            .await?
            .ok_or("Theme not found")?;

        Ok(model.into())
    }
}

impl From<theme_staff::Model> for ThemeStaff {
    fn from(model: theme_staff::Model) -> Self {
        Self {
            id: model.id,
            artist_id: model.artist_id,
            alias: model.alias,
            relevance: model.relevance,
            role: model.role,
            theme_id: model.theme_id,
        }
    }
}
