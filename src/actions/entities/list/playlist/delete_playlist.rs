use loco_rs::app::AppContext;
use sea_orm::ModelTrait;

use crate::entities::list::playlist;
use crate::enums::list::playlistvisibility::PlaylistVisibility;
use crate::events::list::playlist_deleted::PlaylistDeleted;
use crate::initializers::typesense::typesense;
use crate::middlewares::current_user::CurrentUser;
use crate::typesense::documents::playlist_document::PlaylistDocument;
use crate::{AppError, events};

pub struct DeletePlaylistAction;

impl DeletePlaylistAction {
    pub async fn delete(
        ctx: &AppContext,
        playlist: playlist::Model,
        user: &CurrentUser,
    ) -> Result<bool, AppError> {
        let result = playlist.clone().delete(&ctx.db).await?;

        events::dispatch(
            ctx,
            PlaylistDeleted {
                playlist: playlist.clone(),
                user: Some(user.clone()),
            },
        )
        .await;

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
