use std::error::Error as StdError;

use loco_rs::prelude::*;
use sea_orm::{EntityTrait, QueryFilter};
use typesense::prelude::Document;

use crate::entities::content::video;
use crate::scopes::without_trashed;
use crate::typesense::client::TypesenseClient;
use crate::typesense::documents::video_document::{VideoDocument, build_video_documents};
use crate::typesense::index_document;

pub struct SearchIndexVideo;

#[async_trait]
impl Task for SearchIndexVideo {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index-video".to_string(),
            detail: "Import videos from the database into the search index".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
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
            &app_context.db,
            &typesense,
            builder,
            build_video_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
