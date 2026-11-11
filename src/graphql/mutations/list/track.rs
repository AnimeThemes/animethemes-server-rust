use async_graphql::{Context, Error, InputObject, Object, Result, ResultExt};
use loco_rs::app::AppContext;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::AppError;
use crate::actions::entities::list::track::delete_track::DeleteTrackAction;
use crate::actions::entities::list::track::insert_track::{
    InsertTrackAction, InsertTrackActionParameters,
};
use crate::actions::entities::list::track::update_track::{
    UpdateTrackAction, UpdateTrackActionParameters,
};
use crate::entities::list::{playlist, track};
use crate::enums::features::Feature;
use crate::features::functions::FeatureManager;
use crate::graphql::types::list::track::PlaylistTrack;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::list::track::PlaylistTrackPolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(InputObject)]
struct CreatePlaylistTrackInput {
    entry_id: u64,
    video_id: u64,
    position: Option<i32>,
}

#[derive(InputObject)]
struct UpdatePlaylistTrackInput {
    entry_id: Option<u64>,
    video_id: Option<u64>,
    position: Option<i32>,
}

#[derive(Default)]
pub struct PlaylistTrackMutation;

#[Object]
impl PlaylistTrackMutation {
    async fn create_playlist_track(
        &self,
        ctx: &Context<'_>,
        playlist: String,
        input: CreatePlaylistTrackInput,
    ) -> Result<PlaylistTrack> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?;

        let feature_manager = ctx.data_unchecked::<FeatureManager>();

        feature_manager
            .enabled(Feature::AllowPlaylistManagement, Some(&user.user))
            .await
            .extend()?;

        let db = ctx.data::<DatabaseConnection>()?;

        let playlist = playlist::Entity::find()
            .filter(playlist::Column::Hashid.eq(playlist))
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        PlaylistTrackPolicy::check(Some(user), PolicyAction::Create, Some(&playlist))
            .authorize()
            .extend()?;

        let track = InsertTrackAction::insert(
            ctx.data_unchecked::<AppContext>(),
            playlist,
            InsertTrackActionParameters {
                entry_id: input.entry_id,
                video_id: input.video_id,
                position: input.position,
            },
            user,
        )
        .await
        .extend()?;

        Ok(track.into())
    }

    async fn update_playlist_track(
        &self,
        ctx: &Context<'_>,
        id: String,
        playlist: String,
        input: UpdatePlaylistTrackInput,
    ) -> Result<PlaylistTrack> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?;

        let feature_manager = ctx.data_unchecked::<FeatureManager>();

        feature_manager
            .enabled(Feature::AllowPlaylistManagement, Some(&user.user))
            .await
            .extend()?;

        let db = ctx.data::<DatabaseConnection>()?;

        let playlist = playlist::Entity::find()
            .filter(playlist::Column::Hashid.eq(playlist))
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        PlaylistTrackPolicy::check(Some(user), PolicyAction::Update, Some(&playlist))
            .authorize()
            .extend()?;

        let track = track::Entity::find()
            .filter(track::Column::Hashid.eq(id))
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        let track = UpdateTrackAction::update(
            ctx.data_unchecked::<AppContext>(),
            track,
            UpdateTrackActionParameters {
                entry_id: input.entry_id,
                video_id: input.video_id,
                position: input.position,
            },
        )
        .await
        .extend()?;

        Ok(track.into())
    }

    async fn delete_playlist_track(
        &self,
        ctx: &Context<'_>,
        id: String,
        playlist: String,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?;

        let feature_manager = ctx.data_unchecked::<FeatureManager>();

        feature_manager
            .enabled(Feature::AllowPlaylistManagement, Some(&user.user))
            .await
            .extend()?;

        let db = ctx.data::<DatabaseConnection>()?;

        let playlist = playlist::Entity::find()
            .filter(playlist::Column::Hashid.eq(playlist))
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        PlaylistTrackPolicy::check(Some(user), PolicyAction::Delete, Some(&playlist))
            .authorize()
            .extend()?;

        let track = track::Entity::find()
            .filter(track::Column::Hashid.eq(id))
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        DeleteTrackAction::delete(ctx.data_unchecked::<AppContext>(), track)
            .await
            .extend()?;

        Ok(true)
    }
}
