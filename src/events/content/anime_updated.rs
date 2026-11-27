use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::anime;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::middlewares::current_user::CurrentUser;
use crate::services::search_indexer::SearchIndexer;

#[derive(Debug, Clone)]
pub struct AnimeUpdated {
    pub old_anime: anime::Model,
    pub new_anime: anime::Model,
    pub user: Option<CurrentUser>,
}

impl Event for AnimeUpdated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for AnimeUpdated {
    async fn update_typesense_documents(&self, ctx: &AppContext) -> Result<(), AppError> {
        SearchIndexer::reindex_anime(&ctx.db, self.new_anime.clone()).await?;

        Ok(())
    }
}
