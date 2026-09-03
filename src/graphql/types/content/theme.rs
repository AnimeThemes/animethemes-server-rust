use crate::enums::{LocalizedEnum, content::themetype::ThemeType};
use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::content::theme,
    graphql::{
        loaders::content::{
            anime::theme_entries::ThemeEntriesLoader,
            theme::{
                theme_anime::ThemeAnimeLoader, theme_group::ThemeGroupLoader,
                theme_song::ThemeSongLoader,
            },
        },
        types::content::{anime::Anime, entry::Entry, song::Song, themegroup::ThemeGroup},
    },
};

/// Represents an OP or ED sequence for an anime.
///
/// For example, the anime Bakemonogatari has five OP themes and one ED theme.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Theme {
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
    /// The slug that represents the theme.
    pub slug: String,
    /// The type of the sequence
    pub r#type: ThemeType,
    /// The localized string value of the type field
    pub type_localized: String,
}

#[ComplexObject]
impl Theme {
    async fn anime(&self, ctx: &Context<'_>) -> Result<Anime> {
        let loader = ctx.data_unchecked::<DataLoader<ThemeAnimeLoader>>();

        let anime = loader
            .load_one(self.anime_id)
            .await?
            .ok_or("Anime not found")?;

        Ok(anime.into())
    }

    async fn entries(&self, ctx: &Context<'_>) -> Result<Vec<Entry>> {
        let loader = ctx.data_unchecked::<DataLoader<ThemeEntriesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Entry::from).collect())
    }

    async fn song(&self, ctx: &Context<'_>) -> Result<Option<Song>> {
        let Some(song_id) = self.song_id else {
            return Ok(None);
        };

        let loader = ctx.data_unchecked::<DataLoader<ThemeSongLoader>>();

        Ok(loader.load_one(song_id).await?.map(Into::into))
    }

    async fn group(&self, ctx: &Context<'_>) -> Result<Option<ThemeGroup>> {
        let Some(group_id) = self.group_id else {
            return Ok(None);
        };

        let loader = ctx.data_unchecked::<DataLoader<ThemeGroupLoader>>();

        Ok(loader.load_one(group_id).await?.map(Into::into))
    }
}

impl From<theme::Model> for Theme {
    fn from(model: theme::Model) -> Self {
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
