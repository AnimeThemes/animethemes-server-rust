use anyhow::Result;
use sea_orm::{DatabaseConnection, ModelTrait};

use crate::entities::list::playlist;

pub struct DeletePlaylistAction;

impl DeletePlaylistAction {
    pub async fn delete(db: &DatabaseConnection, playlist: playlist::Model) -> Result<bool> {
        let result = playlist.delete(db).await?;

        Ok(result.rows_affected > 0)
    }
}
