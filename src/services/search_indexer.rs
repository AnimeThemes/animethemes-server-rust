use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter, QuerySelect,
};

use crate::AppError;
use crate::entities::content::{anime, artist, entry, series, song, studio, theme, video};
use crate::initializers::typesense::typesense;
use crate::typesense::documents::anime_document::{AnimeDocument, build_anime_documents};
use crate::typesense::documents::artist_document::{ArtistDocument, build_artist_documents};
use crate::typesense::documents::entry_document::{EntryDocument, build_entry_documents};
use crate::typesense::documents::series_document::{SeriesDocument, build_series_documents};
use crate::typesense::documents::song_document::{SongDocument, build_song_documents};
use crate::typesense::documents::studio_document::{StudioDocument, build_studio_documents};
use crate::typesense::documents::theme_document::{ThemeDocument, build_theme_documents};
use crate::typesense::documents::video_document::{VideoDocument, build_video_documents};
use crate::typesense::index_document;

pub struct SearchIndexer;

impl SearchIndexer {
    pub async fn reindex_anime(
        db: &DatabaseConnection,
        anime: anime::Model,
    ) -> Result<(), AppError> {
        let typesense = typesense();
        let anime_id = anime.id;

        let series_ids: Vec<u64> = vec![anime]
            .load_many(series::Entity, db)
            .await?
            .into_iter()
            .flatten()
            .map(|series| series.id)
            .collect();

        let theme_ids: Vec<u64> = theme::Entity::find()
            .select_only()
            .column(theme::Column::Id)
            .filter(theme::Column::AnimeId.eq(anime_id))
            .into_tuple()
            .all(db)
            .await?;

        index_document::<anime::Entity, AnimeDocument, _>(
            db,
            &typesense,
            anime::Entity::find_by_id(anime_id),
            build_anime_documents,
        )
        .await
        .map_err(AppError::internal)?;

        Self::reindex_series(db, series_ids).await?;
        Self::reindex_themes(db, theme_ids).await?;

        Ok(())
    }

    pub async fn reindex_themes(
        db: &DatabaseConnection,
        theme_ids: Vec<u64>,
    ) -> Result<(), AppError> {
        if theme_ids.is_empty() {
            return Ok(());
        }

        let typesense = typesense();

        let entry_ids: Vec<u64> = entry::Entity::find()
            .select_only()
            .column(entry::Column::Id)
            .filter(entry::Column::ThemeId.is_in(theme_ids.iter().copied()))
            .into_tuple()
            .all(db)
            .await?;

        index_document::<theme::Entity, ThemeDocument, _>(
            db,
            &typesense,
            theme::Entity::find().filter(theme::Column::Id.is_in(theme_ids)),
            build_theme_documents,
        )
        .await
        .map_err(AppError::internal)?;

        Self::reindex_entries(db, entry_ids).await?;

        Ok(())
    }

    pub async fn reindex_entries(
        db: &DatabaseConnection,
        entries: Vec<u64>,
    ) -> Result<(), AppError> {
        if entries.is_empty() {
            return Ok(());
        }

        index_document::<entry::Entity, EntryDocument, _>(
            db,
            &typesense(),
            entry::Entity::find().filter(entry::Column::Id.is_in(entries)),
            build_entry_documents,
        )
        .await
        .map_err(AppError::internal)?;

        Ok(())
    }

    pub async fn reindex_videos(db: &DatabaseConnection, videos: Vec<u64>) -> Result<(), AppError> {
        index_document::<video::Entity, VideoDocument, _>(
            db,
            &typesense(),
            video::Entity::find().filter(video::Column::Id.is_in(videos.iter())),
            build_video_documents,
        )
        .await
        .map_err(AppError::internal)?;

        Ok(())
    }

    pub async fn reindex_artists(
        db: &DatabaseConnection,
        artists: Vec<u64>,
    ) -> Result<(), AppError> {
        let typesense = typesense();

        index_document::<artist::Entity, ArtistDocument, _>(
            db,
            &typesense,
            artist::Entity::find().filter(artist::Column::Id.is_in(artists.iter())),
            build_artist_documents,
        )
        .await
        .map_err(AppError::internal)?;

        Ok(())
    }

    pub async fn reindex_series(db: &DatabaseConnection, series: Vec<u64>) -> Result<(), AppError> {
        let typesense = typesense();

        index_document::<series::Entity, SeriesDocument, _>(
            db,
            &typesense,
            series::Entity::find().filter(series::Column::Id.is_in(series.iter())),
            build_series_documents,
        )
        .await
        .map_err(AppError::internal)?;

        Ok(())
    }

    pub async fn reindex_studios(
        db: &DatabaseConnection,
        studio: Vec<u64>,
    ) -> Result<(), AppError> {
        let typesense = typesense();

        index_document::<studio::Entity, StudioDocument, _>(
            db,
            &typesense,
            studio::Entity::find().filter(studio::Column::Id.is_in(studio.iter())),
            build_studio_documents,
        )
        .await
        .map_err(AppError::internal)?;

        Ok(())
    }

    pub async fn reindex_songs(db: &DatabaseConnection, songs: Vec<u64>) -> Result<(), AppError> {
        let typesense = typesense();

        index_document::<song::Entity, SongDocument, _>(
            db,
            &typesense,
            song::Entity::find().filter(song::Column::Id.is_in(songs.iter())),
            build_song_documents,
        )
        .await
        .map_err(AppError::internal)?;

        let theme_ids: Vec<u64> = theme::Entity::find()
            .select_only()
            .column(theme::Column::Id)
            .filter(theme::Column::SongId.is_in(songs.iter()))
            .into_tuple()
            .all(db)
            .await?;

        Self::reindex_themes(db, theme_ids).await?;

        Ok(())
    }
}
