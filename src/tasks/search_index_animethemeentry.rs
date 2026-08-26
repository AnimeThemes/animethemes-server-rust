use loco_rs::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error as StdError;
use typesense::prelude::Document;

use crate::{
    entities::content::animethemeentry,
    scopes::without_trashed,
    typesense::{
        client::TypesenseClient,
        documents::animethemeentry_document::{
            AnimeThemeEntryDocument, build_animethemeentry_documents,
        },
        index_document,
    },
};

pub struct SearchIndexAnimeThemeEntry;

#[async_trait]
impl Task for SearchIndexAnimeThemeEntry {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index_animethemeentry".to_string(),
            detail: "Import animethemeentries from the database into the search index".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let database = app_context
            .shared_store
            .get::<DatabaseConnection>()
            .expect("Database not initialized");

        let typesense = app_context
            .shared_store
            .get::<TypesenseClient>()
            .expect("Typesense not initialized");

        let collection = typesense.collection::<AnimeThemeEntryDocument>();

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
            .create(AnimeThemeEntryDocument::collection_schema())
            .await
            .map_err(
                |err| loco_rs::Error::from(Box::new(err) as Box<dyn StdError + Send + Sync>),
            )?;

        let builder =
            animethemeentry::Entity::find().filter(without_trashed::<animethemeentry::Entity>());

        index_document::<animethemeentry::Entity, AnimeThemeEntryDocument, _>(
            &database,
            &typesense,
            builder,
            build_animethemeentry_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
