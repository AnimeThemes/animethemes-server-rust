use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::entry;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::middlewares::current_user::CurrentUser;
use crate::services::search_indexer::SearchIndexer;

#[derive(Debug, Clone)]
pub struct EntryUpdated {
    pub old_entry: entry::Model,
    pub new_entry: entry::Model,
    pub user: Option<CurrentUser>,
}

impl Event for EntryUpdated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for EntryUpdated {
    async fn update_typesense_documents(&self, ctx: &AppContext) -> Result<(), AppError> {
        SearchIndexer::reindex_entries(&ctx.db, vec![self.new_entry.id]).await?;

        Ok(())
    }
}
