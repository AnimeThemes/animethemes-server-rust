use anyhow::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{entities::list::playlist, enums::list::playlistvisibility::PlaylistVisibility};
use sea_orm::ActiveValue::Set;

pub struct CreatePlaylistActionParameters {
    pub name: String,
    pub visibility: PlaylistVisibility,
    pub description: Option<String>,
    pub user_id: u64,
}

pub struct CreatePlaylistAction;

impl CreatePlaylistAction {
    pub async fn create(
        db: &DatabaseConnection,
        params: CreatePlaylistActionParameters,
    ) -> Result<playlist::Model> {
        let playlist = playlist::ActiveModel {
            name: Set(params.name),
            description: Set(params.description),
            visibility: Set(params.visibility),
            user_id: Set(params.user_id),
            ..Default::default()
        };

        let playlist = playlist.insert(db).await?;

        Ok(playlist)
    }
}
