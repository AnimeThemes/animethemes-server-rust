use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::content::song,
    graphql::{
        loaders::content::song::{
            song_performances::SongPerformancesLoader, song_themes::SongThemesLoader,
        },
        types::content::{performance::Performance, theme::Theme},
    },
};

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

    async fn performances(&self, ctx: &Context<'_>) -> Result<Vec<Performance>> {
        let loader = ctx.data_unchecked::<DataLoader<SongPerformancesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Performance::from).collect())
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
