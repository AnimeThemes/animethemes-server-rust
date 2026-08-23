use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, TransactionTrait};

use crate::{entities::list::track, traits::sortable::Sortable};
use sea_orm::ActiveValue::Set;

pub struct UpdateTrackActionParameters {
    pub entry_id: Option<u64>,
    pub video_id: Option<u64>,
    pub position: Option<i32>,
}

pub struct UpdateTrackAction {}

impl UpdateTrackAction {
    pub async fn update(
        db: &DatabaseConnection,
        track: track::Model,
        params: UpdateTrackActionParameters,
    ) -> Result<track::Model, DbErr> {
        let txn = db.begin().await?;

        let mut active_model: track::ActiveModel = track.clone().into();

        if let Some(entry_id) = params.entry_id {
            active_model.entry_id = Set(Some(entry_id));
        }

        if let Some(video_id) = params.video_id {
            active_model.video_id = Set(Some(video_id));
        }

        let mut track = active_model.update(&txn).await?;

        if let Some(position) = params.position {
            track = track.move_to(&txn, position).await?;
        }

        txn.commit().await?;

        Ok(track)
    }
}
