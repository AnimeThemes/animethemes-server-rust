use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::song;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::initializers::typesense::typesense;
use crate::middlewares::current_user::CurrentUser;
use crate::typesense::documents::song_document::SongDocument;

#[derive(Debug, Clone)]
pub struct SongDeleting {
    pub song: song::Model,
    pub is_force_deleting: bool,
    pub user: Option<CurrentUser>,
}

impl Event for SongDeleting {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for SongDeleting {
    async fn update_typesense_documents(&self, _ctx: &AppContext) -> Result<(), AppError> {
        let typesense = typesense();

        typesense
            .collection::<SongDocument>()
            .document(&self.song.id.to_string())
            .delete()
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
