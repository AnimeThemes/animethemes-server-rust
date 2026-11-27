use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::entry;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::middlewares::current_user::CurrentUser;
use crate::services::search_indexer::SearchIndexer;

#[derive(Debug, Clone)]
pub struct EntryCreated {
    pub entry: entry::Model,
    pub user: Option<CurrentUser>,
}

impl Event for EntryCreated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for EntryCreated {
    async fn update_typesense_documents(&self, ctx: &AppContext) -> Result<(), AppError> {
        SearchIndexer::reindex_entries(&ctx.db, vec![self.entry.id]).await?;

        Ok(())
    }
}
