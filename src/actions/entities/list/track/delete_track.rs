use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, IntoActiveModel, TransactionTrait};

use crate::AppError;
use crate::entities::list::track;
use crate::traits::sortable::Sortable;

pub struct DeleteTrackAction {}

impl DeleteTrackAction {
    pub async fn delete(ctx: &AppContext, track: track::Model) -> Result<(), AppError> {
        let txn = ctx.db.begin().await?;

        track.clone().into_active_model().delete(&txn).await?;

        track.reorder_after_delete(&txn).await?;

        txn.commit().await?;

        Ok(())
    }
}
