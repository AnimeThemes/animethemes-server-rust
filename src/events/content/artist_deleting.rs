use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::artist;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::initializers::typesense::typesense;
use crate::middlewares::current_user::CurrentUser;
use crate::typesense::documents::artist_document::ArtistDocument;

#[derive(Debug, Clone)]
pub struct ArtistDeleting {
    pub artist: artist::Model,
    pub is_force_deleting: bool,
    pub user: Option<CurrentUser>,
}

impl Event for ArtistDeleting {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for ArtistDeleting {
    async fn update_typesense_documents(&self, _ctx: &AppContext) -> Result<(), AppError> {
        let typesense = typesense();

        typesense
            .collection::<ArtistDocument>()
            .document(&self.artist.id.to_string())
            .delete()
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
