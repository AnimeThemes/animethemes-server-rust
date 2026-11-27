use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::series;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::initializers::typesense::typesense;
use crate::middlewares::current_user::CurrentUser;
use crate::typesense::documents::series_document::SeriesDocument;

#[derive(Debug, Clone)]
pub struct SeriesDeleting {
    pub series: series::Model,
    pub is_force_deleting: bool,
    pub user: Option<CurrentUser>,
}

impl Event for SeriesDeleting {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for SeriesDeleting {
    async fn update_typesense_documents(&self, _ctx: &AppContext) -> Result<(), AppError> {
        let typesense = typesense();

        typesense
            .collection::<SeriesDocument>()
            .document(&self.series.id.to_string())
            .delete()
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
