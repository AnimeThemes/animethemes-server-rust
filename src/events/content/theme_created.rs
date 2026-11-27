use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::theme;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::middlewares::current_user::CurrentUser;
use crate::services::search_indexer::SearchIndexer;

#[derive(Debug, Clone)]
pub struct ThemeCreated {
    pub theme: theme::Model,
    pub user: Option<CurrentUser>,
}

impl Event for ThemeCreated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for ThemeCreated {
    async fn update_typesense_documents(&self, ctx: &AppContext) -> Result<(), AppError> {
        SearchIndexer::reindex_themes(&ctx.db, vec![self.theme.id]).await?;

        Ok(())
    }
}
