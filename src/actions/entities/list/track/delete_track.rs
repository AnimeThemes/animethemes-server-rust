use sea_orm::{ActiveModelTrait, IntoActiveModel};
use sea_orm::{DatabaseConnection, TransactionTrait};

use crate::AppError;
use crate::entities::list::track;
use crate::traits::sortable::Sortable;

pub struct DeleteTrackAction {}

impl DeleteTrackAction {
    pub async fn delete(db: &DatabaseConnection, track: track::Model) -> Result<(), AppError> {
        let txn = db.begin().await?;

        track.clone().into_active_model().delete(&txn).await?;

        track.reorder_after_delete(&txn).await?;

        txn.commit().await?;

        Ok(())
    }
}
