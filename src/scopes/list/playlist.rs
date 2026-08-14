use sea_orm::{ColumnTrait, Condition};

use crate::{entities::list::playlist, enums::list::playlistvisibility::PlaylistVisibility};

pub fn public_playlists() -> Condition {
    Condition::all().add(playlist::Column::Visibility.eq(PlaylistVisibility::Public))
}
