use loco_rs::prelude::*;
use sea_orm::{EntityTrait, QueryFilter};
use std::error::Error as StdError;
use typesense::prelude::Document;

use crate::{
    entities::content::song,
    scopes::without_trashed,
    typesense::{
        client::TypesenseClient,
        documents::song_document::{SongDocument, build_song_documents},
        index_document,
    },
};

pub struct SearchIndexSong;

#[async_trait]
impl Task for SearchIndexSong {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index-song".to_string(),
            detail: "Import songs from the database into the search index".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let typesense = app_context
            .shared_store
            .get::<TypesenseClient>()
            .expect("Typesense not initialized");

        let collection = typesense.collection::<SongDocument>();

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
            .create(SongDocument::collection_schema())
            .await
            .map_err(
                |err| loco_rs::Error::from(Box::new(err) as Box<dyn StdError + Send + Sync>),
            )?;

        let builder = song::Entity::find().filter(without_trashed::<song::Entity>());

        index_document::<song::Entity, SongDocument, _>(
            &app_context.db,
            &typesense,
            builder,
            build_song_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
