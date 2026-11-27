use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::entry;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::initializers::typesense::typesense;
use crate::middlewares::current_user::CurrentUser;
use crate::typesense::documents::entry_document::EntryDocument;

#[derive(Debug, Clone)]
pub struct EntryDeleting {
    pub entry: entry::Model,
    pub is_force_deleting: bool,
    pub user: Option<CurrentUser>,
}

impl Event for EntryDeleting {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for EntryDeleting {
    async fn update_typesense_documents(&self, _ctx: &AppContext) -> Result<(), AppError> {
        let typesense = typesense();

        typesense
            .collection::<EntryDocument>()
            .document(&self.entry.id.to_string())
            .delete()
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
