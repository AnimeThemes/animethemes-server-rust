use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, TransactionTrait,
};

use crate::{
    entities::list::{playlist, track},
    traits::sortable::Sortable,
};
use sea_orm::ActiveValue::Set;

pub struct InsertTrackActionParameters {
    pub entry_id: u64,
    pub video_id: u64,
    pub position: Option<i32>,
}

pub struct InsertTrackAction {}

impl InsertTrackAction {
    pub async fn insert(
        db: &DatabaseConnection,
        playlist: playlist::Model,
        params: InsertTrackActionParameters,
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
            entry_id: Set(Some(params.entry_id)),
            video_id: Set(Some(params.video_id)),
            position: Set(max_position),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        if let Some(position) = params.position {
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
}
