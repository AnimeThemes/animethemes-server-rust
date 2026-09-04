use std::env;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, TransactionTrait,
};

use crate::{
    AppError,
    entities::{
        auth::role::Roles,
        list::{playlist, track},
    },
    middlewares::current_user::CurrentUser,
    policies::has_any_role,
    rules::validation_error::ValidationError,
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
    fn validate(params: &InsertTrackActionParameters) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let mut position_errors = Vec::new();

        if let Some(position) = params.position {
            if position < 1 {
                position_errors.push("The position must be greater than 0.");
            }
        }

        if !position_errors.is_empty() {
            errors.push(ValidationError::new("position", position_errors));
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn insert(
        db: &DatabaseConnection,
        playlist: playlist::Model,
        params: InsertTrackActionParameters,
        user: &CurrentUser,
    ) -> Result<track::Model, AppError> {
        Self::validate(&params)?;

        let count = track::Entity::find()
            .filter(track::Column::PlaylistId.eq(playlist.id))
            .count(db)
            .await?;

        let max_tracks: u64 = env::var("PLAYLIST_MAX_TRACKS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1000);

        if count >= max_tracks && !has_any_role(&user.roles, vec![Roles::Admin, Roles::Patron]) {
            return Err(AppError::ForbiddenWithMessage(
                "This playlist has reached the maximum number of tracks".to_string(),
            ));
        }

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
                return Err(AppError::Database(DbErr::Custom(format!(
                    "Invalid track position: {position}"
                ))));
            }

            if position != max_position {
                track = track.move_to(&txn, position).await?;
            }
        }

        txn.commit().await?;

        Ok(track)
    }
}
