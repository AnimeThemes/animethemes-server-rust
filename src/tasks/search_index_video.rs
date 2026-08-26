use loco_rs::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error as StdError;
use typesense::prelude::Document;

use crate::{
    entities::content::video,
    scopes::without_trashed,
    typesense::{
        client::TypesenseClient,
        documents::video_document::{VideoDocument, build_video_documents},
        index_document,
    },
};

pub struct SearchIndexVideo;

#[async_trait]
impl Task for SearchIndexVideo {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index_video".to_string(),
            detail: "Import videos from the database into the search index".to_string(),
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

        let collection = typesense.collection::<VideoDocument>();

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
            .create(VideoDocument::collection_schema())
            .await
            .map_err(
                |err| loco_rs::Error::from(Box::new(err) as Box<dyn StdError + Send + Sync>),
            )?;

        let builder = video::Entity::find().filter(without_trashed::<video::Entity>());

        index_document::<video::Entity, VideoDocument, _>(
            &database,
            &typesense,
            builder,
            build_video_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
