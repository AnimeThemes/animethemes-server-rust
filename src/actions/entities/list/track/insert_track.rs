use std::env;

use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, SelectExt, TransactionTrait,
};
use validator::{Validate, ValidationError};

use crate::entities::HasHashId;
use crate::entities::auth::role::Roles;
use crate::entities::content::entry_videos;
use crate::entities::list::{playlist, track};
use crate::events::list::track_created::TrackCreated;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::has_any_role;
use crate::traits::sortable::Sortable;
use crate::{AppError, events};

#[derive(InputObject, Validate)]
#[graphql(name = "CreatePlaylistTrackInput")]
pub struct InsertTrackActionParameters {
    pub entry_id: u64,
    pub video_id: u64,

    #[validate(range(min = 1, message = "The position must be greater than 0."))]
    pub position: Option<i32>,
}

pub struct InsertTrackAction {}

impl InsertTrackAction {
    async fn validate(
        db: &DatabaseConnection,
        params: &InsertTrackActionParameters,
    ) -> Result<(), AppError> {
        let mut errors = params.validate().err().unwrap_or_default();

        let exists = entry_videos::Entity::find()
            .filter(entry_videos::Column::EntryId.eq(params.entry_id))
            .filter(entry_videos::Column::VideoId.eq(params.video_id))
            .exists(db)
            .await?;

        if !exists {
            let error = ValidationError::new("not_found")
                .with_message("The entry video pair does not exist.".into());

            errors.add("entry_id", error.clone());
            errors.add("video_id", error);
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

        track.update_hashid(&ctx.db).await?;

        events::dispatch(
            ctx,
            TrackCreated {
                track: track.clone(),
                user: Some(user.clone()),
            },
        )
        .await;

        Ok(track)
    }
}
