use crate::{
    entities::list::{playlist, track},
    traits::sortable::Sortable,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect, TransactionTrait,
};

pub struct InsertTrackActionParameters {
    pub entry_id: u64,
    pub video_id: u64,
    pub position: Option<i32>,
}

pub struct UpdateTrackActionParameters {
    pub entry_id: Option<u64>,
    pub video_id: Option<u64>,
    pub position: Option<i32>,
}

pub struct PlaylistTrackAction {}

impl PlaylistTrackAction {
    pub async fn insert(
        db: &DatabaseConnection,
        playlist: playlist::Model,
        parameters: InsertTrackActionParameters,
    ) -> Result<track::Model, DbErr> {
        let txn = db.begin().await?;

        let max_position: i32 = track::Entity::find()
            .select_only()
            .column(track::Column::Position)
            .filter(track::Column::PlaylistId.eq(playlist.id))
            .order_by_desc(track::Column::Position)
            .into_tuple()
            .one(&txn)
            .await?
            .unwrap_or(0)
            + 1;

        let mut track = track::ActiveModel {
            playlist_id: Set(playlist.id),
            entry_id: Set(Some(parameters.entry_id)),
            video_id: Set(Some(parameters.video_id)),
            position: Set(max_position),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        if let Some(position) = parameters.position {
            if position > max_position {
                return Err(DbErr::Custom(format!("Invalid track position: {position}")));
            }

            if position != max_position {
                track = track.move_to(&txn, position).await?;
            }
        }

        txn.commit().await?;

        Ok(track)
    }

    pub async fn update(
        db: &DatabaseConnection,
        track: track::Model,
        parameters: UpdateTrackActionParameters,
    ) -> Result<track::Model, DbErr> {
        let txn = db.begin().await?;

        let mut active_model: track::ActiveModel = track.clone().into();

        if let Some(entry_id) = parameters.entry_id {
            active_model.entry_id = Set(Some(entry_id));
        }

        if let Some(video_id) = parameters.video_id {
            active_model.video_id = Set(Some(video_id));
        }

        let mut track = active_model.update(&txn).await?;

        if let Some(position) = parameters.position {
            track = track.move_to(&txn, position).await?;
        }

        txn.commit().await?;

        Ok(track)
    }

    pub async fn remove(db: &DatabaseConnection, track: track::Model) -> Result<(), DbErr> {
        let txn = db.begin().await?;

        track.clone().into_active_model().delete(&txn).await?;

        track.reorder_after_delete(&txn).await?;

        txn.commit().await?;

        Ok(())
    }
}
