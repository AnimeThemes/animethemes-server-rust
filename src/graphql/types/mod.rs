use async_graphql::{OutputType, SimpleObject};

use crate::graphql::types::content::anime::Anime;
use crate::graphql::types::content::artist::Artist;
use crate::graphql::types::content::series::Series;
use crate::graphql::types::content::song::Song;
use crate::graphql::types::content::studio::Studio;
use crate::graphql::types::content::theme::Theme;
use crate::graphql::types::content::video::Video;
use crate::graphql::types::list::playlist::Playlist;

pub mod admin;
pub mod auth;
pub mod content;
pub mod document;
pub mod list;
pub mod user;

#[derive(SimpleObject)]
#[graphql(concrete(name = "AnimePagination", params(Anime)))]
#[graphql(concrete(name = "ArtistPagination", params(Artist)))]
#[graphql(concrete(name = "ThemePagination", params(Theme)))]
#[graphql(concrete(name = "PlaylistPagination", params(Playlist)))]
#[graphql(concrete(name = "SeriesPagination", params(Series)))]
#[graphql(concrete(name = "SongPagination", params(Song)))]
#[graphql(concrete(name = "StudioPagination", params(Studio)))]
#[graphql(concrete(name = "VideoPagination", params(Video)))]
pub struct OffsetPagination<T: OutputType> {
    /// The data for the current page.
    pub data: Vec<T>,
    /// Information to aid in pagination.
    pub page_info: OffsetPageInfo,
}

#[derive(SimpleObject)]
pub struct OffsetPageInfo {
    /// The total number of items.
    pub total: u64,
    /// The offset of the current page.
    pub offset: u64,
    /// The number of items per page.
    pub first: u64,
    /// When paginating backwards, are there more items? Note: Paginating backwards is not supported.
    pub has_previous_page: bool,
    /// When paginating forwards, are there more items?
    pub has_next_page: bool,
}
