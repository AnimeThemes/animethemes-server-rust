use loco_rs::app::AppContext;

use crate::AppError;

pub trait UpdateTypesenseDocuments {
    fn update_typesense_documents(
        &self,
        _ctx: &AppContext,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}
