use loco_rs::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error as StdError;
use typesense::prelude::Document;

use crate::{
    entities::content::studio,
    scopes::without_trashed,
    typesense::{
        client::TypesenseClient,
        documents::studio_document::{StudioDocument, build_studio_documents},
        index_document,
    },
};

pub struct SearchIndexStudio;

#[async_trait]
impl Task for SearchIndexStudio {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index-studio".to_string(),
            detail: "Import studios from the database into the search index".to_string(),
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

        let collection = typesense.collection::<StudioDocument>();

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
            .create(StudioDocument::collection_schema())
            .await
            .map_err(
                |err| loco_rs::Error::from(Box::new(err) as Box<dyn StdError + Send + Sync>),
            )?;

        let builder = studio::Entity::find().filter(without_trashed::<studio::Entity>());

        index_document::<studio::Entity, StudioDocument, _>(
            &database,
            &typesense,
            builder,
            build_studio_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
