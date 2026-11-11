use std::env;
use std::time::Duration;

use async_graphql::dataloader::DataLoader;
use async_graphql::{EmptySubscription, SchemaBuilder};
use sea_orm::DatabaseConnection;

use crate::graphql::loaders::admin::featuredtheme_entry::FeaturedEntryLoader;
use crate::graphql::loaders::admin::featuredtheme_user::FeaturedThemeUserLoader;
use crate::graphql::loaders::admin::featuredtheme_video::FeaturedThemeVideoLoader;
use crate::graphql::loaders::auth::user::user_favorites::UserFavoritesLoader;
use crate::graphql::loaders::auth::user::user_playlists::UserPlaylistsLoader;
use crate::graphql::loaders::auth::user::user_ratings::UserRatingsLoader;
use crate::graphql::loaders::auth::user::user_roles::UserRolesLoader;
use crate::graphql::loaders::auth::user::user_watchhistory::UserWatchHistoryLoader;
use crate::graphql::loaders::content::anime::anime_series::AnimeSeriesLoader;
use crate::graphql::loaders::content::anime::anime_studios::AnimeStudiosLoader;
use crate::graphql::loaders::content::anime::anime_synonyms::AnimeSynonymsLoader;
use crate::graphql::loaders::content::anime::anime_themes::ThemesLoader;
use crate::graphql::loaders::content::anime::theme_entries::ThemeEntriesLoader;
use crate::graphql::loaders::content::artist::artist_groups::ArtistGroupsLoader;
use crate::graphql::loaders::content::artist::artist_members::ArtistMembersLoader;
use crate::graphql::loaders::content::artist::artist_membersongstaffs::ArtistMemberSongStaffsLoader;
use crate::graphql::loaders::content::artist::artist_songstaffs::ArtistSongStaffsLoader;
use crate::graphql::loaders::content::artist::artist_synonyms::ArtistSynonymsLoader;
use crate::graphql::loaders::content::artist::artist_themestaffs::ArtistThemeStaffsLoader;
use crate::graphql::loaders::content::entry::entry_theme::EntryThemeLoader;
use crate::graphql::loaders::content::entry::entry_videos::EntryVideosLoader;
use crate::graphql::loaders::content::imageable::ImageableLoader;
use crate::graphql::loaders::content::resourceable::ResourceableLoader;
use crate::graphql::loaders::content::series::series_anime::SeriesAnimeLoader;
use crate::graphql::loaders::content::song::song_staffs::SongStaffsLoader;
use crate::graphql::loaders::content::song::song_themes::SongThemesLoader;
use crate::graphql::loaders::content::songstaff::songstaff_artist::SongStaffArtistLoader;
use crate::graphql::loaders::content::songstaff::songstaff_member::SongStaffMemberLoader;
use crate::graphql::loaders::content::songstaff::songstaff_song::SongStaffSongLoader;
use crate::graphql::loaders::content::studio::studio_anime::StudioAnimeLoader;
use crate::graphql::loaders::content::theme::theme_anime::ThemeAnimeLoader;
use crate::graphql::loaders::content::theme::theme_group::ThemeGroupLoader;
use crate::graphql::loaders::content::theme::theme_song::ThemeSongLoader;
use crate::graphql::loaders::content::theme::theme_staffs::ThemeStaffsLoader;
use crate::graphql::loaders::content::themestaff::themestaff_artist::ThemeStaffArtistLoader;
use crate::graphql::loaders::content::themestaff::themestaff_theme::ThemeStaffThemeLoader;
use crate::graphql::loaders::content::video::video_audio::VideoAudioLoader;
use crate::graphql::loaders::content::video::video_entries::VideoThemeEntriesLoader;
use crate::graphql::loaders::content::video::video_script::VideoScriptLoader;
use crate::graphql::loaders::content::video::video_tracks::VideoTracksLoader;
use crate::graphql::loaders::document::page_page::PagePageLoader;
use crate::graphql::loaders::list::playlist::playlist_tracks::PlaylistTracksLoader;
use crate::graphql::loaders::list::playlist::playlist_tracks_count::PlaylistTracksCountLoader;
use crate::graphql::loaders::list::playlist::playlist_user::PlaylistUserLoader;
use crate::graphql::loaders::list::playlist::track_entry::TrackEntryLoader;
use crate::graphql::loaders::list::playlist::track_playlist::TrackPlaylistLoader;
use crate::graphql::loaders::list::playlist::track_video::TrackVideoLoader;
use crate::graphql::loaders::user::favorite::favorite_entry::FavoriteEntryLoader;
use crate::graphql::loaders::user::favorite::favorite_user::FavoriteUserLoader;
use crate::graphql::loaders::user::rating::rating_entry::RatingEntryLoader;
use crate::graphql::loaders::user::watchhistory::watchhistory_entry::WatchHistoryEntryLoader;
use crate::graphql::loaders::user::watchhistory::watchhistory_video::WatchHistoryVideoLoader;
use crate::graphql::mutation::Mutation;

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
            .data(loader(UserRatingsLoader { db: db.clone() }))
            .data(loader(UserRolesLoader { db: db.clone() }))
            .data(loader(UserWatchHistoryLoader { db: db.clone() }))
            .data(loader(UserFavoritesLoader { db: db.clone() }))
            .data(loader(RatingEntryLoader { db: db.clone() }))
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
            .data(loader(ArtistSongStaffsLoader { db: db.clone() }))
            .data(loader(ArtistMemberSongStaffsLoader { db: db.clone() }))
            .data(loader(ArtistThemeStaffsLoader { db: db.clone() }))
            .data(loader(SeriesAnimeLoader { db: db.clone() }))
            .data(loader(SongThemesLoader { db: db.clone() }))
            .data(loader(SongStaffsLoader { db: db.clone() }))
            .data(loader(StudioAnimeLoader { db: db.clone() }))
            .data(loader(SongStaffArtistLoader { db: db.clone() }))
            .data(loader(SongStaffMemberLoader { db: db.clone() }))
            .data(loader(SongStaffSongLoader { db: db.clone() }))
            .data(loader(ThemeStaffsLoader { db: db.clone() }))
            .data(loader(ThemeStaffArtistLoader { db: db.clone() }))
            .data(loader(ThemeStaffThemeLoader { db: db.clone() }))
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
