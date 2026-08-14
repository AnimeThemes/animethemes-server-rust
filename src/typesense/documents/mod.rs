pub mod anime_document;
pub mod animetheme_document;
pub mod animethemeentry_document;
pub mod artist_document;
pub mod playlist_document;
pub mod series_document;
pub mod song_document;
pub mod studio_document;
pub mod video_document;

pub trait HasId {
    fn id(&self) -> &str;
}
