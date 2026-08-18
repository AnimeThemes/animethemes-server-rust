use crate::{
    entities::{
        content::{anime, animetheme, artist, series, song, studio, video},
        list::playlist,
    },
    typesense::{
        client::TypesenseClient,
        search::{
            search_anime, search_animethemes, search_artists, search_playlists, search_series,
            search_songs, search_studios, search_videos,
        },
    },
};
use async_graphql::{Context, Object, ObjectType, Result};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::graphql::types::{
    content::{
        anime::Anime, animetheme::AnimeTheme, artist::Artist, series::Series, song::Song,
        studio::Studio, video::Video,
    },
    list::playlist::Playlist,
};

struct Search {
    term: String,
    first: i32,
}

/// Returns a listing of resources that match a given search term.
#[Object]
impl Search {
    /// The anime results of the search
    async fn anime(&self, ctx: &Context<'_>) -> Result<Vec<Anime>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        Ok(convert_type(
            search_anime(
                db,
                typesense,
                anime::Entity::find(),
                self.term.clone(),
                self.first,
            )
            .await?,
        ))
    }

    /// The artist results of the search
    async fn artists(&self, ctx: &Context<'_>) -> Result<Vec<Artist>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        Ok(convert_type(
            search_artists(
                db,
                typesense,
                artist::Entity::find(),
                self.term.clone(),
                self.first,
            )
            .await?,
        ))
    }

    /// The theme results of the search
    async fn animethemes(&self, ctx: &Context<'_>) -> Result<Vec<AnimeTheme>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        Ok(convert_type(
            search_animethemes(
                db,
                typesense,
                animetheme::Entity::find(),
                self.term.clone(),
                self.first,
            )
            .await?,
        ))
    }

    /// The playlist results of the search
    async fn playlists(&self, ctx: &Context<'_>) -> Result<Vec<Playlist>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        Ok(convert_type(
            search_playlists(
                db,
                typesense,
                playlist::Entity::find(),
                self.term.clone(),
                self.first,
            )
            .await?,
        ))
    }

    /// The series results of the search
    async fn series(&self, ctx: &Context<'_>) -> Result<Vec<Series>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        Ok(convert_type(
            search_series(
                db,
                typesense,
                series::Entity::find(),
                self.term.clone(),
                self.first,
            )
            .await?,
        ))
    }

    /// The song results of the search
    async fn songs(&self, ctx: &Context<'_>) -> Result<Vec<Song>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        Ok(convert_type(
            search_songs(
                db,
                typesense,
                song::Entity::find(),
                self.term.clone(),
                self.first,
            )
            .await?,
        ))
    }

    /// The studio results of the search
    async fn studios(&self, ctx: &Context<'_>) -> Result<Vec<Studio>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        Ok(convert_type(
            search_studios(
                db,
                typesense,
                studio::Entity::find(),
                self.term.clone(),
                self.first,
            )
            .await?,
        ))
    }

    /// The video results of the search
    async fn videos(&self, ctx: &Context<'_>) -> Result<Vec<Video>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let typesense = ctx.data::<TypesenseClient>()?;

        Ok(convert_type(
            search_videos(
                db,
                typesense,
                video::Entity::find(),
                self.term.clone(),
                self.first,
            )
            .await?,
        ))
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
    ) -> Result<Search> {
        Ok(Search {
            term: search,
            first: first,
        })
    }
}

fn convert_type<T, M>(models: Vec<M>) -> Vec<T>
where
    T: ObjectType + From<M>,
{
    models.into_iter().map(T::from).collect()
}
