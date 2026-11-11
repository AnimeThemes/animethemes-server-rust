use std::env;

use anyhow::Context;
use loco_rs::app::AppContext;
use loco_rs::prelude::BackgroundWorker;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};

use crate::entities::auth::role::Roles;
use crate::entities::list::playlist;
use crate::enums::features::Feature;
use crate::enums::list::playlistvisibility::PlaylistVisibility;
use crate::features::functions::FeatureManager;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::has_any_role;
use crate::rules::validation_error::ValidationError;
use crate::typesense::client::typesense;
use crate::typesense::documents::playlist_document::{PlaylistDocument, build_playlist_documents};
use crate::workers::discord_notification::{
    DiscordNotificationWorker, DiscordNotificationWorkerArgs,
};
use crate::{AppError, discord};

pub struct CreatePlaylistActionParameters {
    pub name: String,
    pub visibility: PlaylistVisibility,
    pub description: Option<String>,
    pub user_id: u64,
}

pub struct CreatePlaylistAction;

impl CreatePlaylistAction {
    fn validate(params: &CreatePlaylistActionParameters) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let mut name_errors = Vec::new();
        let mut description_errors = Vec::new();

        if !(1usize..=192).contains(&params.name.chars().count()) {
            name_errors.push("The name must be between 1 and 192 characters.");
        }

        if let Some(description) = &params.description {
            if !(1usize..=1000).contains(&description.chars().count()) {
                description_errors.push("The description must be between 1 and 1000 characters.");
            }
        }

        if !name_errors.is_empty() {
            errors.push(ValidationError::new("name", name_errors));
        }

        if !description_errors.is_empty() {
            errors.push(ValidationError::new("description", description_errors));
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn create(
        ctx: &AppContext,
        params: CreatePlaylistActionParameters,
        user: &CurrentUser,
    ) -> Result<playlist::Model, AppError> {
        Self::validate(&params)?;

        let count = playlist::Entity::find()
            .filter(playlist::Column::UserId.eq(params.user_id))
            .count(&ctx.db)
            .await?;

        let max_playlists: u64 = env::var("USER_MAX_PLAYLISTS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1000);

        if count >= max_playlists && !has_any_role(&user.roles, vec![Roles::Admin, Roles::Patron]) {
            return Err(AppError::ForbiddenWithMessage(
                "You have reached the maximum number of playlists".to_string(),
            ));
        }

        let playlist = playlist::ActiveModel {
            name: Set(params.name),
            description: Set(params.description),
            visibility: Set(params.visibility),
            user_id: Set(params.user_id),
            ..Default::default()
        };

        let playlist = playlist.insert(&ctx.db).await?;

        Self::update_search(&ctx.db, &playlist).await?;

        let feature_manager = ctx.shared_store.get::<FeatureManager>();

        if let Some(feature_manager) = feature_manager
            && feature_manager.is_enabled(Feature::AllowDiscordNotifications, None, false)
        {
            DiscordNotificationWorker::perform_later(
                ctx,
                DiscordNotificationWorkerArgs {
                    channel_id: discord::playlist::Playlist::get_discord_channel(),
                    message: serde_json::to_value(
                        discord::playlist::Playlist::new(playlist.clone(), Some(user.clone()))
                            .created_message(),
                    )
                    .unwrap(),
                },
            )
            .await
            .map_err(AppError::internal)?;
        }

        Ok(playlist)
    }

    async fn update_search(
        db: &DatabaseConnection,
        playlist: &playlist::Model,
    ) -> Result<(), AppError> {
        if playlist.visibility != PlaylistVisibility::Public {
            return Ok(());
        }

        let document = build_playlist_documents(vec![playlist.clone()], db)
            .await
            .map_err(AppError::internal)?
            .into_iter()
            .next()
            .context("Failed")
            .map_err(AppError::internal)?;

        let typesense = typesense();

        typesense
            .collection::<PlaylistDocument>()
            .documents()
            .create(&document, None)
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
