use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::studio;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::middlewares::current_user::CurrentUser;
use crate::services::search_indexer::SearchIndexer;

#[derive(Debug, Clone)]
pub struct StudioCreated {
    pub studio: studio::Model,
    pub user: Option<CurrentUser>,
}

impl Event for StudioCreated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for StudioCreated {
    async fn update_typesense_documents(&self, ctx: &AppContext) -> Result<(), AppError> {
        SearchIndexer::reindex_studios(&ctx.db, vec![self.studio.id]).await?;

        Ok(())
    }
}
