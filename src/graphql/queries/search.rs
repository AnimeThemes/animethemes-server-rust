use std::vec;

use crate::{
    entities::{
        content::{anime, animetheme, artist, series, song, studio, video},
        list::playlist,
    },
    graphql::{
        enums::search_sort::{
            SearchAnimeSort, SearchAnimeThemeSort, SearchArtistSort, SearchPlaylistSort,
            SearchSeriesSort, SearchSort, SearchStudioSort,
        },
        types::{OffsetPageInfo, OffsetPagination},
    },
    scopes::list::playlist::public_playlists,
    typesense::{
        client::TypesenseClient,
        search::{
            search_anime, search_animethemes, search_artists, search_playlists, search_series,
            search_songs, search_studios, search_videos,
        },
    },
};
use crate::{
    enums::content::{animeformat::AnimeFormat, animeseason::AnimeSeason, themetype::ThemeType},
    typesense::search::OffsetPageInfo as OffsetPageInfoTypesense,
};
use async_graphql::{Context, InputObject, Object, ObjectType, Result};
use sea_orm::{ActiveEnum, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

use crate::graphql::types::{
    content::{
        anime::Anime, animetheme::AnimeTheme, artist::Artist, series::Series, song::Song,
        studio::Studio, video::Video,
    },
    list::playlist::Playlist,
};

pub struct Search {
    term: String,
    first: i32,
    page: i32,
}

#[derive(InputObject, Default)]
struct SearchAnimeFilterInput {
    title_romaji_prefix: Option<String>,
    season: Option<AnimeSeason>,
    year: Option<i16>,
    format: Option<AnimeFormat>,
}

impl SearchAnimeFilterInput {
    pub fn to_typesense_filter(&self) -> Option<String> {
        let mut filters = Vec::new();

        if let Some(title_prefix) = &self.title_romaji_prefix {
            filters.push(format!("title:={}*", title_prefix));
        }

        if let Some(season) = self.season {
            filters.push(format!("season:={}", season.to_value()));
        }

        if let Some(year) = self.year {
            filters.push(format!("year:={}", year));
        }

        if let Some(format) = self.format {
            filters.push(format!("format:={}", format.to_value()));
        }

        (!filters.is_empty()).then(|| filters.join(" && "))
    }
}

#[derive(InputObject, Default)]
struct SearchArtistFilterInput {
    name_main_prefix: Option<String>,
}

impl SearchArtistFilterInput {
    pub fn to_typesense_filter(&self) -> Option<String> {
        let mut filters = Vec::new();

        if let Some(name_main_prefix) = &self.name_main_prefix {
            filters.push(format!("name:={}*", name_main_prefix));
        }

        (!filters.is_empty()).then(|| filters.join(" && "))
    }
}

#[derive(InputObject, Default)]
struct SearchSeriesFilterInput {
    title_romaji_prefix: Option<String>,
}

impl SearchSeriesFilterInput {
    pub fn to_typesense_filter(&self) -> Option<String> {
        let mut filters = Vec::new();

        if let Some(title_romaji_prefix) = &self.title_romaji_prefix {
            filters.push(format!("title:={}*", title_romaji_prefix));
        }

        (!filters.is_empty()).then(|| filters.join(" && "))
    }
}

#[derive(InputObject, Default)]
struct SearchStudioFilterInput {
    name_prefix: Option<String>,
}

impl SearchStudioFilterInput {
    pub fn to_typesense_filter(&self) -> Option<String> {
        let mut filters = Vec::new();

        if let Some(name_prefix) = &self.name_prefix {
            filters.push(format!("name:={}*", name_prefix));
        }

        (!filters.is_empty()).then(|| filters.join(" && "))
    }
}

#[derive(InputObject, Default)]
struct SearchAnimeThemeFilterInput {
    song_title_romaji_prefix: Option<String>,
    r#type: Option<ThemeType>,
}

impl SearchAnimeThemeFilterInput {
    pub fn to_typesense_filter(&self) -> Option<String> {
        let mut filters = Vec::new();

        if let Some(song_title_romaji_prefix) = &self.song_title_romaji_prefix {
            filters.push(format!("song_title:={}*", song_title_romaji_prefix));
        }

        if let Some(r#type) = &self.r#type {
            filters.push(format!("type:{}", r#type.to_value()));
        }

        (!filters.is_empty()).then(|| filters.join(" && "))
    }
}

/// Returns a listing of resources that match a given search term.
#[Object]
impl Search {
    /// The anime results of the search
    async fn anime(
        &self,
        ctx: &Context<'_>,
        filter: Option<SearchAnimeFilterInput>,
        sort: Option<Vec<SearchAnimeSort>>,
    ) -> Result<OffsetPagination<Anime>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let sort_by = sort
            .map(|s| {
                s.into_iter()
                    .map(|s| s.sort_key().to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let result = search_anime(
            db,
            typesense,
            anime::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
            filter.as_ref().and_then(|f| f.to_typesense_filter()),
            sort_by,
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }

    /// The artist results of the search
    async fn artists(
        &self,
        ctx: &Context<'_>,
        filter: Option<SearchArtistFilterInput>,
        sort: Option<Vec<SearchArtistSort>>,
    ) -> Result<OffsetPagination<Artist>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let sort_by = sort
            .map(|s| {
                s.into_iter()
                    .map(|s| s.sort_key().to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let result = search_artists(
            db,
            typesense,
            artist::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
            filter.as_ref().and_then(|f| f.to_typesense_filter()),
            sort_by,
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }

    /// The theme results of the search
    async fn animethemes(
        &self,
        ctx: &Context<'_>,
        filter: Option<SearchAnimeThemeFilterInput>,
        sort: Option<Vec<SearchAnimeThemeSort>>,
    ) -> Result<OffsetPagination<AnimeTheme>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let sort_by = sort
            .map(|s| {
                s.into_iter()
                    .map(|s| s.sort_key().to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let result = search_animethemes(
            db,
            typesense,
            animetheme::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
            filter.as_ref().and_then(|f| f.to_typesense_filter()),
            sort_by,
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }

    /// The playlist results of the search
    async fn playlists(
        &self,
        ctx: &Context<'_>,
        sort: Option<Vec<SearchPlaylistSort>>,
    ) -> Result<OffsetPagination<Playlist>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let sort_by = sort
            .map(|s| {
                s.into_iter()
                    .map(|s| s.sort_key().to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let builder = playlist::Entity::find().filter(public_playlists());

        let result = search_playlists(
            db,
            typesense,
            builder,
            self.term.clone(),
            self.first,
            self.page,
            None,
            sort_by,
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }

    /// The series results of the search
    async fn series(
        &self,
        ctx: &Context<'_>,
        filter: Option<SearchSeriesFilterInput>,
        sort: Option<Vec<SearchSeriesSort>>,
    ) -> Result<OffsetPagination<Series>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let sort_by = sort
            .map(|s| {
                s.into_iter()
                    .map(|s| s.sort_key().to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let result = search_series(
            db,
            typesense,
            series::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
            filter.as_ref().and_then(|f| f.to_typesense_filter()),
            sort_by,
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }

    /// The song results of the search
    async fn songs(&self, ctx: &Context<'_>) -> Result<OffsetPagination<Song>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let result = search_songs(
            db,
            typesense,
            song::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
            None,
            vec![],
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }

    /// The studio results of the search
    async fn studios(
        &self,
        ctx: &Context<'_>,
        filter: Option<SearchStudioFilterInput>,
        sort: Option<Vec<SearchStudioSort>>,
    ) -> Result<OffsetPagination<Studio>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let sort_by = sort
            .map(|s| {
                s.into_iter()
                    .map(|s| s.sort_key().to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let result = search_studios(
            db,
            typesense,
            studio::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
            filter.as_ref().and_then(|f| f.to_typesense_filter()),
            sort_by,
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }

    /// The video results of the search
    async fn videos(&self, ctx: &Context<'_>) -> Result<OffsetPagination<Video>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let result = search_videos(
            db,
            typesense,
            video::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
            None,
            vec![],
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }
}

#[derive(Default)]
pub struct SearchQuery;

#[Object]
impl SearchQuery {
    async fn search(
        &self,
        _ctx: &Context<'_>,
        search: String,
        #[graphql(default = 10)] first: i32,
        #[graphql(default = 1)] page: i32,
    ) -> Result<Search> {
        Ok(Search {
            term: search,
            first: first,
            page: page,
        })
    }
}

fn convert_type<T, M>(models: Vec<M>, page_info: OffsetPageInfoTypesense) -> OffsetPagination<T>
where
    M: ModelTrait,
    T: ObjectType + From<M>,
{
    OffsetPagination {
        data: models.into_iter().map(T::from).collect(),
        page_info: OffsetPageInfo {
            total: page_info.total,
            offset: page_info.offset,
            first: page_info.first,
            has_previous_page: page_info.has_previous_page,
            has_next_page: page_info.has_next_page,
        },
    }
}
