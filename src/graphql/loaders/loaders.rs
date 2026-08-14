use async_graphql::{EmptySubscription, SchemaBuilder, dataloader::DataLoader};
use sea_orm::DatabaseConnection;

use crate::graphql::{
    loaders::{
        admin::{
            featuredtheme_entry::FeaturedThemeEntryLoader,
            featuredtheme_user::FeaturedThemeUserLoader,
            featuredtheme_video::FeaturedThemeVideoLoader,
        },
        auth::user::{user_playlists::UserPlaylistsLoader, user_roles::UserRolesLoader},
        content::{
            anime::{
                anime_series::AnimeSeriesLoader, anime_studios::AnimeStudiosLoader,
                anime_synonyms::AnimeSynonymsLoader, anime_theme_entries::AnimeThemeEntriesLoader,
                anime_themes::AnimeThemesLoader,
            },
            animetheme::{
                animetheme_anime::AnimeThemeAnimeLoader, animetheme_group::AnimeThemeGroupLoader,
                animetheme_song::AnimeThemeSongLoader,
            },
            animethemeentry::{
                animethemeentry_theme::AnimeThemeEntryThemeLoader,
                animethemeentry_videos::AnimeThemeEntryVideosLoader,
            },
            artist::{
                artist_groups::ArtistGroupsLoader, artist_members::ArtistMembersLoader,
                artist_performances::ArtistPerformancesLoader,
            },
            imageable::ImageableLoader,
            performance::{
                performance_artist::PerformanceArtistLoader,
                performance_member::PerformanceMemberLoader,
                performance_song::PerformanceSongLoader,
            },
            resourceable::ResourceableLoader,
            song::song_performances::SongPerformancesLoader,
            studio::studio_anime::StudioAnimeLoader,
            video::{video_audio::VideoAudioLoader, video_script::VideoScriptLoader},
        },
        document::page_page::PagePageLoader,
        list::playlist::{
            playlist_tracks::PlaylistTracksLoader,
            playlist_tracks_count::PlaylistTracksCountLoader, playlist_user::PlaylistUserLoader,
            track_entry::TrackEntryLoader, track_track::TrackTrackLoader,
            track_video::TrackVideoLoader,
        },
    },
    mutation::Mutation,
};

fn loader<L>(loader: L) -> DataLoader<L>
where
    L: Send + Sync + 'static,
{
    DataLoader::new(loader, tokio::spawn)
}

pub trait RegisterLoaders {
    fn register_loaders(self, db: DatabaseConnection) -> Self;
}

impl<Query> RegisterLoaders for SchemaBuilder<Query, Mutation, EmptySubscription> {
    fn register_loaders(self, db: DatabaseConnection) -> Self {
        self.data(loader(FeaturedThemeEntryLoader { db: db.clone() }))
            .data(loader(FeaturedThemeUserLoader { db: db.clone() }))
            .data(loader(FeaturedThemeVideoLoader { db: db.clone() }))
            .data(loader(UserPlaylistsLoader { db: db.clone() }))
            .data(loader(UserRolesLoader { db: db.clone() }))
            .data(loader(AnimeSynonymsLoader { db: db.clone() }))
            .data(loader(AnimeThemesLoader { db: db.clone() }))
            .data(loader(AnimeThemeAnimeLoader { db: db.clone() }))
            .data(loader(AnimeThemeSongLoader { db: db.clone() }))
            .data(loader(AnimeThemeGroupLoader { db: db.clone() }))
            .data(loader(AnimeThemeEntriesLoader { db: db.clone() }))
            .data(loader(AnimeThemeEntryThemeLoader { db: db.clone() }))
            .data(loader(AnimeThemeEntryVideosLoader { db: db.clone() }))
            .data(loader(AnimeSeriesLoader { db: db.clone() }))
            .data(loader(AnimeStudiosLoader { db: db.clone() }))
            .data(loader(ArtistGroupsLoader { db: db.clone() }))
            .data(loader(ArtistMembersLoader { db: db.clone() }))
            .data(loader(ArtistPerformancesLoader { db: db.clone() }))
            .data(loader(SongPerformancesLoader { db: db.clone() }))
            .data(loader(StudioAnimeLoader { db: db.clone() }))
            .data(loader(PerformanceArtistLoader { db: db.clone() }))
            .data(loader(PerformanceMemberLoader { db: db.clone() }))
            .data(loader(PerformanceSongLoader { db: db.clone() }))
            .data(loader(VideoAudioLoader { db: db.clone() }))
            .data(loader(VideoScriptLoader { db: db.clone() }))
            .data(loader(ImageableLoader { db: db.clone() }))
            .data(loader(ResourceableLoader { db: db.clone() }))
            .data(loader(PlaylistUserLoader { db: db.clone() }))
            .data(loader(PlaylistTracksLoader { db: db.clone() }))
            .data(loader(PlaylistTracksCountLoader { db: db.clone() }))
            .data(loader(TrackEntryLoader { db: db.clone() }))
            .data(loader(TrackVideoLoader { db: db.clone() }))
            .data(loader(TrackTrackLoader { db: db.clone() }))
            .data(loader(PagePageLoader { db: db.clone() }))
    }
}
