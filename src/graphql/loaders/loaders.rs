use std::{env, time::Duration};

use async_graphql::{EmptySubscription, SchemaBuilder, dataloader::DataLoader};
use sea_orm::DatabaseConnection;

use crate::graphql::{
    loaders::{
        admin::{
            featuredtheme_entry::FeaturedEntryLoader, featuredtheme_user::FeaturedThemeUserLoader,
            featuredtheme_video::FeaturedThemeVideoLoader,
        },
        auth::user::{
            user_favorites::UserFavoritesLoader, user_playlists::UserPlaylistsLoader,
            user_roles::UserRolesLoader, user_watchhistory::UserWatchHistoryLoader,
        },
        content::{
            anime::{
                anime_series::AnimeSeriesLoader, anime_studios::AnimeStudiosLoader,
                anime_synonyms::AnimeSynonymsLoader, anime_themes::ThemesLoader,
                theme_entries::ThemeEntriesLoader,
            },
            artist::{
                artist_groups::ArtistGroupsLoader,
                artist_memberperformances::ArtistMemberPerformancesLoader,
                artist_members::ArtistMembersLoader, artist_performances::ArtistPerformancesLoader,
                artist_synonyms::ArtistSynonymsLoader,
            },
            entry::{entry_theme::EntryThemeLoader, entry_videos::EntryVideosLoader},
            imageable::ImageableLoader,
            performance::{
                performance_artist::PerformanceArtistLoader,
                performance_member::PerformanceMemberLoader,
                performance_song::PerformanceSongLoader,
            },
            resourceable::ResourceableLoader,
            series::series_anime::SeriesAnimeLoader,
            song::{song_performances::SongPerformancesLoader, song_themes::SongThemesLoader},
            studio::studio_anime::StudioAnimeLoader,
            theme::{
                theme_anime::ThemeAnimeLoader, theme_group::ThemeGroupLoader,
                theme_song::ThemeSongLoader,
            },
            video::{
                video_audio::VideoAudioLoader, video_entries::VideoThemeEntriesLoader,
                video_script::VideoScriptLoader, video_tracks::VideoTracksLoader,
            },
        },
        document::page_page::PagePageLoader,
        list::playlist::{
            playlist_tracks::PlaylistTracksLoader,
            playlist_tracks_count::PlaylistTracksCountLoader, playlist_user::PlaylistUserLoader,
            track_entry::TrackEntryLoader, track_playlist::TrackPlaylistLoader,
            track_video::TrackVideoLoader,
        },
        user::{
            favorite::{favorite_entry::FavoriteEntryLoader, favorite_user::FavoriteUserLoader},
            watchhistory::{
                watchhistory_entry::WatchHistoryEntryLoader,
                watchhistory_video::WatchHistoryVideoLoader,
            },
        },
    },
    mutation::Mutation,
};

fn loader<L>(loader: L) -> DataLoader<L>
where
    L: Send + Sync + 'static,
{
    let delay = env::var("DATALOADER_DELAY_MS").unwrap_or(1.to_string());
    let max_batch_size = env::var("DATALOADER_MAX_BATCH_SIZE").unwrap_or(1000.to_string());

    DataLoader::new(loader, tokio::spawn)
        .delay(Duration::from_millis(delay.parse().unwrap_or(1)))
        .max_batch_size(max_batch_size.parse().unwrap_or(1000))
}

pub trait RegisterLoaders {
    fn register_loaders(self, db: DatabaseConnection) -> Self;
}

impl<Query> RegisterLoaders for SchemaBuilder<Query, Mutation, EmptySubscription> {
    fn register_loaders(self, db: DatabaseConnection) -> Self {
        self.data(loader(FeaturedEntryLoader { db: db.clone() }))
            .data(loader(FeaturedThemeUserLoader { db: db.clone() }))
            .data(loader(FeaturedThemeVideoLoader { db: db.clone() }))
            .data(loader(UserPlaylistsLoader { db: db.clone() }))
            .data(loader(UserRolesLoader { db: db.clone() }))
            .data(loader(UserWatchHistoryLoader { db: db.clone() }))
            .data(loader(UserFavoritesLoader { db: db.clone() }))
            .data(loader(WatchHistoryEntryLoader { db: db.clone() }))
            .data(loader(WatchHistoryVideoLoader { db: db.clone() }))
            .data(loader(FavoriteEntryLoader { db: db.clone() }))
            .data(loader(FavoriteUserLoader { db: db.clone() }))
            .data(loader(AnimeSynonymsLoader { db: db.clone() }))
            .data(loader(ThemesLoader { db: db.clone() }))
            .data(loader(ThemeAnimeLoader { db: db.clone() }))
            .data(loader(ThemeSongLoader { db: db.clone() }))
            .data(loader(ThemeGroupLoader { db: db.clone() }))
            .data(loader(ThemeEntriesLoader { db: db.clone() }))
            .data(loader(EntryThemeLoader { db: db.clone() }))
            .data(loader(EntryVideosLoader { db: db.clone() }))
            .data(loader(AnimeSeriesLoader { db: db.clone() }))
            .data(loader(AnimeStudiosLoader { db: db.clone() }))
            .data(loader(ArtistSynonymsLoader { db: db.clone() }))
            .data(loader(ArtistGroupsLoader { db: db.clone() }))
            .data(loader(ArtistMembersLoader { db: db.clone() }))
            .data(loader(ArtistPerformancesLoader { db: db.clone() }))
            .data(loader(ArtistMemberPerformancesLoader { db: db.clone() }))
            .data(loader(SeriesAnimeLoader { db: db.clone() }))
            .data(loader(SongThemesLoader { db: db.clone() }))
            .data(loader(SongPerformancesLoader { db: db.clone() }))
            .data(loader(StudioAnimeLoader { db: db.clone() }))
            .data(loader(PerformanceArtistLoader { db: db.clone() }))
            .data(loader(PerformanceMemberLoader { db: db.clone() }))
            .data(loader(PerformanceSongLoader { db: db.clone() }))
            .data(loader(VideoThemeEntriesLoader { db: db.clone() }))
            .data(loader(VideoAudioLoader { db: db.clone() }))
            .data(loader(VideoScriptLoader { db: db.clone() }))
            .data(loader(VideoTracksLoader { db: db.clone() }))
            .data(loader(ImageableLoader { db: db.clone() }))
            .data(loader(ResourceableLoader { db: db.clone() }))
            .data(loader(PlaylistUserLoader { db: db.clone() }))
            .data(loader(PlaylistTracksLoader { db: db.clone() }))
            .data(loader(PlaylistTracksCountLoader { db: db.clone() }))
            .data(loader(TrackPlaylistLoader { db: db.clone() }))
            .data(loader(TrackEntryLoader { db: db.clone() }))
            .data(loader(TrackVideoLoader { db: db.clone() }))
            .data(loader(PagePageLoader { db: db.clone() }))
    }
}
