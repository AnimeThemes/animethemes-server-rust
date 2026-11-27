use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::list::playlist;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::middlewares::current_user::CurrentUser;
use crate::services::search_indexer::SearchIndexer;

#[derive(Debug, Clone)]
pub struct PlaylistUpdated {
    pub old_playlist: playlist::Model,
    pub new_playlist: playlist::Model,
    pub user: Option<CurrentUser>,
}

impl Event for PlaylistUpdated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for PlaylistUpdated {
    async fn update_typesense_documents(&self, ctx: &AppContext) -> Result<(), AppError> {
        SearchIndexer::reindex_playlist(&ctx.db, vec![self.new_playlist.id]).await?;

        Ok(())
    }
}
