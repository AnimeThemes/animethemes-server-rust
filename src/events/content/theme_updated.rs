use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::theme;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::middlewares::current_user::CurrentUser;
use crate::services::search_indexer::SearchIndexer;

#[derive(Debug, Clone)]
pub struct ThemeUpdated {
    pub old_theme: theme::Model,
    pub new_theme: theme::Model,
    pub user: Option<CurrentUser>,
}

impl Event for ThemeUpdated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for ThemeUpdated {
    async fn update_typesense_documents(&self, ctx: &AppContext) -> Result<(), AppError> {
        SearchIndexer::reindex_themes(&ctx.db, vec![self.new_theme.id]).await?;

        Ok(())
    }
}
