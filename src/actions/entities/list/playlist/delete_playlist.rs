use anyhow::Result;
use sea_orm::{DatabaseConnection, ModelTrait};

use crate::{
    AppError,
    entities::list::playlist,
    enums::list::playlistvisibility::PlaylistVisibility,
    typesense::{client::typesense, documents::playlist_document::PlaylistDocument},
};

pub struct DeletePlaylistAction;

impl DeletePlaylistAction {
    pub async fn delete(db: &DatabaseConnection, playlist: playlist::Model) -> Result<bool> {
        let result = playlist.clone().delete(db).await?;

        Self::update_search(&playlist).await?;

        Ok(result.rows_affected > 0)
    }

    async fn update_search(playlist: &playlist::Model) -> Result<(), AppError> {
        if playlist.visibility != PlaylistVisibility::Public {
            return Ok(());
        }

        let typesense = typesense();

        typesense
            .collection::<PlaylistDocument>()
            .document(playlist.id.to_string())
            .delete()
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
