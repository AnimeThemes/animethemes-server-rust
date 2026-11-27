use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::song;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::middlewares::current_user::CurrentUser;
use crate::services::search_indexer::SearchIndexer;

#[derive(Debug, Clone)]
pub struct SongCreated {
    pub song: song::Model,
    pub user: Option<CurrentUser>,
}

impl Event for SongCreated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for SongCreated {
    async fn update_typesense_documents(&self, ctx: &AppContext) -> Result<(), AppError> {
        SearchIndexer::reindex_songs(&ctx.db, vec![self.song.id]).await?;

        Ok(())
    }
}
