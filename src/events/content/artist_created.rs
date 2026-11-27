use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::artist;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::middlewares::current_user::CurrentUser;
use crate::services::search_indexer::SearchIndexer;

#[derive(Debug, Clone)]
pub struct ArtistCreated {
    pub artist: artist::Model,
    pub user: Option<CurrentUser>,
}

impl Event for ArtistCreated {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for ArtistCreated {
    async fn update_typesense_documents(&self, ctx: &AppContext) -> Result<(), AppError> {
        SearchIndexer::reindex_artists(&ctx.db, vec![self.artist.id]).await?;

        Ok(())
    }
}
