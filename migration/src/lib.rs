pub use sea_orm_migration::prelude::*;

mod m20260812_231526_create_users_table;
mod m20260812_235939_create_videos_table;
mod m20260813_000507_create_announcements_table;
mod m20260813_000608_create_anime_table;
mod m20260813_000925_create_series_table;
mod m20260813_001026_create_songs_table;
mod m20260813_001122_create_artists_table;
mod m20260813_001246_create_resources_table;
mod m20260813_002050_create_groups_table;
mod m20260813_002131_create_anime_themes_table;
mod m20260813_002331_create_anime_theme_entries_table;
mod m20260813_002652_create_anime_series_table;
mod m20260813_003551_create_anime_theme_entry_video_table;
mod m20260813_003837_create_artist_member_table;
mod m20260813_004225_create_images_table;
mod m20260813_004321_create_studios_table;
mod m20260813_011602_create_anime_studio_table;
mod m20260813_011704_create_pages_table;
mod m20260813_012147_create_audios_table;
mod m20260813_013404_create_video_scripts_table;
mod m20260813_021040_create_playlists_table;
mod m20260813_022143_create_playlist_tracks_table;
mod m20260813_023144_create_featured_themes_table;
mod m20260813_023457_create_performances_table;
mod m20260813_023839_create_resourceables_table;
mod m20260813_024141_create_imageables_table;
mod m20260813_024405_create_synonyms_table;
mod m20260813_024620_create_roles_table;
mod m20260813_025156_create_page_roles_table;
mod m20260824_174646_create_feature_flags_table;
mod m20260824_224413_create_watch_history_table;
mod m20260825_203853_create_password_reset_tokens_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260812_231526_create_users_table::Migration),
            Box::new(m20260812_235939_create_videos_table::Migration),
            Box::new(m20260813_000507_create_announcements_table::Migration),
            Box::new(m20260813_000608_create_anime_table::Migration),
            Box::new(m20260813_000925_create_series_table::Migration),
            Box::new(m20260813_001026_create_songs_table::Migration),
            Box::new(m20260813_001122_create_artists_table::Migration),
            Box::new(m20260813_001246_create_resources_table::Migration),
            Box::new(m20260813_002050_create_groups_table::Migration),
            Box::new(m20260813_002131_create_anime_themes_table::Migration),
            Box::new(m20260813_002331_create_anime_theme_entries_table::Migration),
            Box::new(m20260813_002652_create_anime_series_table::Migration),
            Box::new(m20260813_003551_create_anime_theme_entry_video_table::Migration),
            Box::new(m20260813_003837_create_artist_member_table::Migration),
            Box::new(m20260813_004225_create_images_table::Migration),
            Box::new(m20260813_004321_create_studios_table::Migration),
            Box::new(m20260813_011602_create_anime_studio_table::Migration),
            Box::new(m20260813_011704_create_pages_table::Migration),
            Box::new(m20260813_012147_create_audios_table::Migration),
            Box::new(m20260813_013404_create_video_scripts_table::Migration),
            Box::new(m20260813_021040_create_playlists_table::Migration),
            Box::new(m20260813_022143_create_playlist_tracks_table::Migration),
            Box::new(m20260813_023144_create_featured_themes_table::Migration),
            Box::new(m20260813_023457_create_performances_table::Migration),
            Box::new(m20260813_023839_create_resourceables_table::Migration),
            Box::new(m20260813_024141_create_imageables_table::Migration),
            Box::new(m20260813_024405_create_synonyms_table::Migration),
            Box::new(m20260813_024620_create_roles_table::Migration),
            Box::new(m20260813_025156_create_page_roles_table::Migration),
            Box::new(m20260824_174646_create_feature_flags_table::Migration),
            Box::new(m20260824_224413_create_watch_history_table::Migration),
            Box::new(m20260825_203853_create_password_reset_tokens_table::Migration),
        ]
    }
}
