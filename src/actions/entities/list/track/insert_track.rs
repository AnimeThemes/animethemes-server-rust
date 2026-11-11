use std::env;

use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, SelectExt, TransactionTrait,
};

use crate::AppError;
use crate::entities::auth::role::Roles;
use crate::entities::content::entry_videos;
use crate::entities::list::{playlist, track};
use crate::middlewares::current_user::CurrentUser;
use crate::policies::has_any_role;
use crate::rules::validation_error::ValidationError;
use crate::traits::sortable::Sortable;

pub struct InsertTrackActionParameters {
    pub entry_id: u64,
    pub video_id: u64,
    pub position: Option<i32>,
}

pub struct InsertTrackAction {}

impl InsertTrackAction {
    async fn validate(
        db: &DatabaseConnection,
        params: &InsertTrackActionParameters,
    ) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let mut entry_id_errors = Vec::new();
        let mut video_id_errors = Vec::new();
        let mut position_errors = Vec::new();

        let entry_video_exists = entry_videos::Entity::find()
            .filter(entry_videos::Column::EntryId.eq(params.entry_id))
            .filter(entry_videos::Column::VideoId.eq(params.video_id))
            .exists(db)
            .await?;

        if !entry_video_exists {
            entry_id_errors.push("The entry video pair does not exist.");
            video_id_errors.push("The entry video pair does not exist.");
        }

        if let Some(position) = params.position {
            if position < 1 {
                position_errors.push("The position must be greater than 0.");
            }
        }

        if !entry_id_errors.is_empty() {
            errors.push(ValidationError::new("entry_id", entry_id_errors));
        }

        if !video_id_errors.is_empty() {
            errors.push(ValidationError::new("video_id", video_id_errors));
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
        ctx: &AppContext,
        playlist: playlist::Model,
        params: InsertTrackActionParameters,
        user: &CurrentUser,
    ) -> Result<track::Model, AppError> {
        Self::validate(&ctx.db, &params).await?;

        let count = track::Entity::find()
            .filter(track::Column::PlaylistId.eq(playlist.id))
            .count(&ctx.db)
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

        let txn = ctx.db.begin().await?;

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
