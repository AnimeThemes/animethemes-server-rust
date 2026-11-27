use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::studio;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::initializers::typesense::typesense;
use crate::middlewares::current_user::CurrentUser;
use crate::typesense::documents::studio_document::StudioDocument;

#[derive(Debug, Clone)]
pub struct StudioDeleting {
    pub studio: studio::Model,
    pub is_force_deleting: bool,
    pub user: Option<CurrentUser>,
}

impl Event for StudioDeleting {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for StudioDeleting {
    async fn update_typesense_documents(&self, _ctx: &AppContext) -> Result<(), AppError> {
        let typesense = typesense();

        typesense
            .collection::<StudioDocument>()
            .document(&self.studio.id.to_string())
            .delete()
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
