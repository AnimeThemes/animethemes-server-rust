use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::content::song,
    graphql::{
        loaders::content::song::{
            song_animethemes::SongAnimeThemesLoader, song_performances::SongPerformancesLoader,
        },
        types::content::{animetheme::AnimeTheme, performance::Performance},
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

/// Represents the composition that accompanies an AnimeTheme.
///
/// For example, Staple Stable is the song for the Bakemonogatari OP1 AnimeTheme.
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
    async fn animethemes(&self, ctx: &Context<'_>) -> Result<Vec<AnimeTheme>> {
        let loader = ctx.data_unchecked::<DataLoader<SongAnimeThemesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(AnimeTheme::from).collect())
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
