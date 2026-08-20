use animethemes_server_rust::enums::{LocalizedEnum, content::themetype::ThemeType};
use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::content::animetheme,
    graphql::{
        loaders::content::{
            anime::anime_theme_entries::AnimeThemeEntriesLoader,
            animetheme::{
                animetheme_anime::AnimeThemeAnimeLoader, animetheme_group::AnimeThemeGroupLoader,
                animetheme_song::AnimeThemeSongLoader,
            },
        },
        types::content::{
            anime::Anime, animethemeentry::AnimeThemeEntry, song::Song, themegroup::ThemeGroup,
        },
    },
};

/// Represents an OP or ED sequence for an anime.
///
/// For example, the anime Bakemonogatari has five OP anime themes and one ED anime theme.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct AnimeTheme {
    /// The primary key of the resource
    pub id: u64,
    #[graphql(skip)]
    pub anime_id: u64,
    #[graphql(skip)]
    pub group_id: Option<u64>,
    /// The numeric ordering of the theme
    pub sequence: Option<i32>,
    #[graphql(skip)]
    pub song_id: Option<u64>,
    /// The slug that represents the anime theme.
    pub slug: String,
    /// The type of the sequence
    pub r#type: ThemeType,
    /// The localized string value of the type field
    pub type_localized: String,
}

#[ComplexObject]
impl AnimeTheme {
    async fn anime(&self, ctx: &Context<'_>) -> Result<Anime> {
        let loader = ctx.data::<DataLoader<AnimeThemeAnimeLoader>>()?;

        let anime = loader
            .load_one(self.anime_id)
            .await?
            .ok_or("Anime not found")?;

        Ok(anime.into())
    }

    async fn animethemeentries(&self, ctx: &Context<'_>) -> Result<Vec<AnimeThemeEntry>> {
        let loader = ctx.data::<DataLoader<AnimeThemeEntriesLoader>>()?;

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(AnimeThemeEntry::from).collect())
    }

    async fn song(&self, ctx: &Context<'_>) -> Result<Option<Song>> {
        let Some(song_id) = self.song_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<AnimeThemeSongLoader>>()?;

        Ok(loader.load_one(song_id).await?.map(Into::into))
    }

    async fn group(&self, ctx: &Context<'_>) -> Result<Option<ThemeGroup>> {
        let Some(group_id) = self.group_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<AnimeThemeGroupLoader>>()?;

        Ok(loader.load_one(group_id).await?.map(Into::into))
    }
}

impl From<animetheme::Model> for AnimeTheme {
    fn from(model: animetheme::Model) -> Self {
        Self {
            id: model.id,
            anime_id: model.anime_id,
            group_id: model.group_id,
            sequence: model.sequence,
            song_id: model.song_id,
            slug: model.slug,
            r#type: model.r#type,
            type_localized: model.r#type.localize().to_string(),
        }
    }
}
