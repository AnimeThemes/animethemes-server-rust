use anyhow::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{entities::list::playlist, enums::list::playlistvisibility::PlaylistVisibility};
use sea_orm::ActiveValue::Set;

pub struct UpdatePlaylistActionParameters {
    pub name: Option<String>,
    pub visibility: Option<PlaylistVisibility>,
    pub description: Option<String>,
}

pub struct UpdatePlaylistAction;

impl UpdatePlaylistAction {
    pub async fn update(
        db: &DatabaseConnection,
        playlist: playlist::Model,
        params: UpdatePlaylistActionParameters,
    ) -> Result<playlist::Model> {
        let mut playlist = playlist::ActiveModel {
            id: Set(playlist.id),
            ..Default::default()
        };

        if let Some(name) = params.name {
            playlist.name = Set(name);
        }

        if let Some(description) = params.description {
            playlist.description = Set(Some(description));
        }

        if let Some(visibility) = params.visibility {
            playlist.visibility = Set(visibility);
        }

        let playlist = playlist.update(db).await?;

        Ok(playlist)
    }
}
