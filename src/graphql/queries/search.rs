use crate::{
    entities::{
        content::{anime, animetheme, artist, series, song, studio, video},
        list::playlist,
    },
    graphql::types::{OffsetPageInfo, OffsetPagination},
    typesense::{
        client::TypesenseClient,
        search::{
            search_anime, search_animethemes, search_artists, search_playlists, search_series,
            search_songs, search_studios, search_videos,
        },
    },
};
use animethemes_server_rust::typesense::search::OffsetPageInfo as OffsetPageInfoTypesense;
use async_graphql::{Context, Object, ObjectType, Result};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};

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

/// Returns a listing of resources that match a given search term.
#[Object]
impl Search {
    /// The anime results of the search
    async fn anime(&self, ctx: &Context<'_>) -> Result<OffsetPagination<Anime>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let result = search_anime(
            db,
            typesense,
            anime::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }

    /// The artist results of the search
    async fn artists(&self, ctx: &Context<'_>) -> Result<OffsetPagination<Artist>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let result = search_artists(
            db,
            typesense,
            artist::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }

    /// The theme results of the search
    async fn animethemes(&self, ctx: &Context<'_>) -> Result<OffsetPagination<AnimeTheme>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let result = search_animethemes(
            db,
            typesense,
            animetheme::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }

    /// The playlist results of the search
    async fn playlists(&self, ctx: &Context<'_>) -> Result<OffsetPagination<Playlist>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let result = search_playlists(
            db,
            typesense,
            playlist::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }

    /// The series results of the search
    async fn series(&self, ctx: &Context<'_>) -> Result<OffsetPagination<Series>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let result = search_series(
            db,
            typesense,
            series::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
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
        )
        .await?;

        Ok(convert_type(result.data, result.page_info))
    }

    /// The studio results of the search
    async fn studios(&self, ctx: &Context<'_>) -> Result<OffsetPagination<Studio>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        let result = search_studios(
            db,
            typesense,
            studio::Entity::find(),
            self.term.clone(),
            self.first,
            self.page,
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
