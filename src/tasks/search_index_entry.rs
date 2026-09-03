use loco_rs::prelude::*;
use sea_orm::{EntityTrait, QueryFilter};
use std::error::Error as StdError;
use typesense::prelude::Document;

use crate::{
    entities::content::entry,
    scopes::without_trashed,
    typesense::{
        client::TypesenseClient,
        documents::entry_document::{EntryDocument, build_entry_documents},
        index_document,
    },
};

pub struct SearchIndexEntry;

#[async_trait]
impl Task for SearchIndexEntry {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index-entry".to_string(),
            detail: "Import entries from the database into the search index".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let typesense = app_context
            .shared_store
            .get::<TypesenseClient>()
            .expect("Typesense not initialized");

        let collection = typesense.collection::<EntryDocument>();

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
            .create(EntryDocument::collection_schema())
            .await
            .map_err(
                |err| loco_rs::Error::from(Box::new(err) as Box<dyn StdError + Send + Sync>),
            )?;

        let builder = entry::Entity::find().filter(without_trashed::<entry::Entity>());

        index_document::<entry::Entity, EntryDocument, _>(
            &app_context.db,
            &typesense,
            builder,
            build_entry_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
