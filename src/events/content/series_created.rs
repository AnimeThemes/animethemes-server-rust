use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::series;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::middlewares::current_user::CurrentUser;
use crate::services::search_indexer::SearchIndexer;

#[derive(Debug, Clone)]
pub struct SeriesCreated {
    pub series: series::Model,
    pub user: Option<CurrentUser>,
}

impl Event for SeriesCreated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for SeriesCreated {
    async fn update_typesense_documents(&self, ctx: &AppContext) -> Result<(), AppError> {
        SearchIndexer::reindex_series(&ctx.db, vec![self.series.id]).await?;

        Ok(())
    }
}
