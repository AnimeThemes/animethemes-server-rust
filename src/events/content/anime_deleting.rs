use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::entities::content::anime;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;
use crate::initializers::typesense::typesense;
use crate::middlewares::current_user::CurrentUser;
use crate::typesense::documents::anime_document::AnimeDocument;

#[derive(Debug, Clone)]
pub struct AnimeDeleting {
    pub anime: anime::Model,
    pub is_force_deleting: bool,
    pub user: Option<CurrentUser>,
}

impl Event for AnimeDeleting {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl UpdateTypesenseDocuments for AnimeDeleting {
    async fn update_typesense_documents(&self, _ctx: &AppContext) -> Result<(), AppError> {
        let typesense = typesense();

        typesense
            .collection::<AnimeDocument>()
            .document(&self.anime.id.to_string())
            .delete()
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
