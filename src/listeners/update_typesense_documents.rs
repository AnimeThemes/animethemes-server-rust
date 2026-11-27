use loco_rs::app::AppContext;
use mod_events::Event;

use crate::AppError;
use crate::events::update_typesense_documents::UpdateTypesenseDocuments;

pub async fn handle<E: UpdateTypesenseDocuments + Event>(
    ctx: &AppContext,
    event: &E,
) -> Result<(), AppError> {
    event.update_typesense_documents(ctx).await?;

    Ok(())
}
