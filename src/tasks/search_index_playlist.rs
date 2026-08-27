use loco_rs::prelude::*;
use sea_orm::EntityTrait;
use std::error::Error as StdError;
use typesense::prelude::Document;

use crate::{
    entities::list::playlist,
    typesense::{
        client::TypesenseClient,
        documents::playlist_document::{PlaylistDocument, build_playlist_documents},
        index_document,
    },
};

pub struct SearchIndexPlaylist;

#[async_trait]
impl Task for SearchIndexPlaylist {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index-playlist".to_string(),
            detail: "Import playlists from the database into the search index".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let typesense = app_context
            .shared_store
            .get::<TypesenseClient>()
            .expect("Typesense not initialized");

        let collection = typesense.collection::<PlaylistDocument>();

        match collection.retrieve().await {
            Ok(_) => {
                collection.delete().await.map_err(|err| {
                    loco_rs::Error::from(Box::new(err) as Box<dyn StdError + Send + Sync>)
                })?;
            }
            Err(_) => {}
        }

        typesense
            .collections()
            .create(PlaylistDocument::collection_schema())
            .await
            .map_err(
                |err| loco_rs::Error::from(Box::new(err) as Box<dyn StdError + Send + Sync>),
            )?;

        let builder = playlist::Entity::find();

        index_document::<playlist::Entity, PlaylistDocument, _>(
            &app_context.db,
            &typesense,
            builder,
            build_playlist_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
