use anyhow::{Context, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{
    AppError,
    entities::list::playlist,
    enums::list::playlistvisibility::PlaylistVisibility,
    typesense::{
        client::typesense,
        documents::playlist_document::{PlaylistDocument, build_playlist_documents},
    },
};
use sea_orm::ActiveValue::Set;

pub struct UpdatePlaylistActionParameters {
    pub name: Option<String>,
    pub visibility: Option<PlaylistVisibility>,
    pub description: Option<String>,
}

pub struct UpdatePlaylistAction;

impl UpdatePlaylistAction {
    pub async fn update(
        db: &DatabaseConnection,
        playlist: playlist::Model,
        params: UpdatePlaylistActionParameters,
    ) -> Result<playlist::Model> {
        let mut playlist = playlist::ActiveModel {
            id: Set(playlist.id),
            ..Default::default()
        };

        if let Some(name) = params.name {
            playlist.name = Set(name);
        }

        if let Some(description) = params.description {
            playlist.description = Set(Some(description));
        }

        if let Some(visibility) = params.visibility {
            playlist.visibility = Set(visibility);
        }

        let playlist = playlist.update(db).await?;

        Self::update_search(db, &playlist).await?;

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
            .upsert(&document, None)
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
