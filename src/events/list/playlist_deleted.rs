use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::list::playlist;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::initializers::typesense::typesense;
use crate::middlewares::current_user::CurrentUser;
use crate::typesense::documents::playlist_document::PlaylistDocument;

#[derive(Debug, Clone)]
pub struct PlaylistDeleted {
    pub playlist: playlist::Model,
    pub user: Option<CurrentUser>,
}

impl Event for PlaylistDeleted {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for PlaylistDeleted {
    async fn update_typesense_documents(&self, _ctx: &AppContext) -> Result<(), AppError> {
        let typesense = typesense();

        typesense
            .collection::<PlaylistDocument>()
            .document(self.playlist.id.to_string())
            .delete()
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
