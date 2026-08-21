use anyhow::Result;
use async_graphql::Error;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Select,
    sea_query::{CaseStatement, SimpleExpr},
};
use typesense::{models::SearchParameters, prelude::Document};

use crate::{
    entities::{
        content::{anime, animetheme, artist, series, song, studio, video},
        list::playlist,
    },
    scopes::list::playlist::public_playlists,
    typesense::{
        client::TypesenseClient,
        documents::{
            HasId,
            anime_document::{self, AnimeDocument},
            animetheme_document::{self, AnimeThemeDocument},
            artist_document::{self, ArtistDocument},
            playlist_document::{self, PlaylistDocument},
            series_document::{self, SeriesDocument},
            song_document::{self, SongDocument},
            studio_document::{self, StudioDocument},
            video_document::{self, VideoDocument},
        },
    },
};

pub async fn search_anime(
    db: &DatabaseConnection,
    typesense: &TypesenseClient,
    builder: Select<anime::Entity>,
    term: String,
    first: i32,
    page: i32,
    sort_by: Vec<String>,
) -> Result<OffsetPagination<anime::Model>> {
    search::<anime::Entity, AnimeDocument>(
        db,
        typesense,
        builder,
        anime::Column::Id,
        term,
        first as u64,
        page as u64,
        sort_by,
        anime_document::QUERY_BY,
        anime_document::QUERY_BY_WEIGHTS,
    )
    .await
}

pub async fn search_artists(
    db: &DatabaseConnection,
    typesense: &TypesenseClient,
    builder: Select<artist::Entity>,
    term: String,
    first: i32,
    page: i32,
    sort_by: Vec<String>,
) -> Result<OffsetPagination<artist::Model>> {
    search::<artist::Entity, ArtistDocument>(
        db,
        typesense,
        builder,
        artist::Column::Id,
        term,
        first as u64,
        page as u64,
        sort_by,
        artist_document::QUERY_BY,
        artist_document::QUERY_BY_WEIGHTS,
    )
    .await
}

pub async fn search_animethemes(
    db: &DatabaseConnection,
    typesense: &TypesenseClient,
    builder: Select<animetheme::Entity>,
    term: String,
    first: i32,
    page: i32,
    sort_by: Vec<String>,
) -> Result<OffsetPagination<animetheme::Model>> {
    search::<animetheme::Entity, AnimeThemeDocument>(
        db,
        typesense,
        builder,
        animetheme::Column::Id,
        term,
        first as u64,
        page as u64,
        sort_by,
        animetheme_document::QUERY_BY,
        animetheme_document::QUERY_BY_WEIGHTS,
    )
    .await
}

pub async fn search_playlists(
    db: &DatabaseConnection,
    typesense: &TypesenseClient,
    builder: Select<playlist::Entity>,
    term: String,
    first: i32,
    page: i32,
    sort_by: Vec<String>,
) -> Result<OffsetPagination<playlist::Model>> {
    search::<playlist::Entity, PlaylistDocument>(
        db,
        typesense,
        builder.filter(public_playlists()),
        playlist::Column::Id,
        term,
        first as u64,
        page as u64,
        sort_by,
        playlist_document::QUERY_BY,
        playlist_document::QUERY_BY_WEIGHTS,
    )
    .await
}

pub async fn search_series(
    db: &DatabaseConnection,
    typesense: &TypesenseClient,
    builder: Select<series::Entity>,
    term: String,
    first: i32,
    page: i32,
    sort_by: Vec<String>,
) -> Result<OffsetPagination<series::Model>> {
    search::<series::Entity, SeriesDocument>(
        db,
        typesense,
        builder,
        series::Column::Id,
        term,
        first as u64,
        page as u64,
        sort_by,
        series_document::QUERY_BY,
        series_document::QUERY_BY_WEIGHTS,
    )
    .await
}

pub async fn search_songs(
    db: &DatabaseConnection,
    typesense: &TypesenseClient,
    builder: Select<song::Entity>,
    term: String,
    first: i32,
    page: i32,
    sort_by: Vec<String>,
) -> Result<OffsetPagination<song::Model>> {
    search::<song::Entity, SongDocument>(
        db,
        typesense,
        builder,
        song::Column::Id,
        term,
        first as u64,
        page as u64,
        sort_by,
        song_document::QUERY_BY,
        song_document::QUERY_BY_WEIGHTS,
    )
    .await
}

pub async fn search_studios(
    db: &DatabaseConnection,
    typesense: &TypesenseClient,
    builder: Select<studio::Entity>,
    term: String,
    first: i32,
    page: i32,
    sort_by: Vec<String>,
) -> Result<OffsetPagination<studio::Model>> {
    search::<studio::Entity, StudioDocument>(
        db,
        typesense,
        builder,
        studio::Column::Id,
        term,
        first as u64,
        page as u64,
        sort_by,
        studio_document::QUERY_BY,
        studio_document::QUERY_BY_WEIGHTS,
    )
    .await
}

pub async fn search_videos(
    db: &DatabaseConnection,
    typesense: &TypesenseClient,
    builder: Select<video::Entity>,
    term: String,
    first: i32,
    page: i32,
    sort_by: Vec<String>,
) -> Result<OffsetPagination<video::Model>> {
    search::<video::Entity, VideoDocument>(
        db,
        typesense,
        builder,
        video::Column::Id,
        term,
        first as u64,
        page as u64,
        sort_by,
        video_document::QUERY_BY,
        video_document::QUERY_BY_WEIGHTS,
    )
    .await
}

pub struct OffsetPagination<T> {
    pub data: Vec<T>,
    pub page_info: OffsetPageInfo,
}

pub struct OffsetPageInfo {
    pub total: u64,
    pub offset: u64,
    pub first: u64,
    pub has_previous_page: bool,
    pub has_next_page: bool,
}

async fn search<E, D>(
    db: &DatabaseConnection,
    typesense: &TypesenseClient,
    builder: Select<E>,
    id_column: E::Column,
    term: String,
    first: u64,
    page: u64,
    sort_by: Vec<String>,
    query_by: &str,
    query_by_weights: &str,
) -> Result<OffsetPagination<E::Model>>
where
    E: EntityTrait,
    D: Document + HasId,
{
    let offset = page as u64 * first;

    let documents = typesense
        .collection::<D>()
        .documents()
        .search(
            SearchParameters::builder()
                .q(term)
                .sort_by(sort_by.join(","))
                .query_by(query_by)
                .query_by_weights(query_by_weights)
                .page(page as i32)
                .per_page(first as i32)
                .build(),
        )
        .await
        .map_err(|error| Error::new(error.to_string()))
        .unwrap();

    let total = documents.found.unwrap_or_default() as u64;

    let ids: Vec<String> = documents
        .hits
        .unwrap_or_default()
        .into_iter()
        .filter_map(|hit| hit.document)
        .filter_map(|document| document.id().parse::<String>().ok())
        .collect();

    let page_info = OffsetPageInfo {
        total,
        offset,
        first,
        has_previous_page: page > 1,
        has_next_page: (offset + ids.len() as u64) < total,
    };

    if ids.is_empty() {
        return Ok(OffsetPagination {
            data: Vec::new(),
            page_info,
        });
    }

    let order: SimpleExpr = ids
        .iter()
        .enumerate()
        .fold(CaseStatement::new(), |case, (index, id)| {
            case.case(id_column.eq(id), index as i32)
        })
        .into();

    let models = builder
        .filter(id_column.is_in(&ids))
        .order_by_asc(order)
        .all(db)
        .await?;

    Ok(OffsetPagination {
        data: models.into_iter().map(Into::into).collect(),

        page_info,
    })
}
