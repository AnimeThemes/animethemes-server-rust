use loco_rs::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error as StdError;
use typesense::prelude::Document;

use crate::{
    entities::content::artist,
    scopes::without_trashed,
    typesense::{
        client::TypesenseClient,
        documents::artist_document::{ArtistDocument, build_artist_documents},
        index_document,
    },
};

pub struct SearchIndexArtist;

#[async_trait]
impl Task for SearchIndexArtist {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index_artist".to_string(),
            detail: "Import artists from the database into the search index".to_string(),
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

        let collection = typesense.collection::<ArtistDocument>();

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
            .create(ArtistDocument::collection_schema())
            .await
            .map_err(
                |err| loco_rs::Error::from(Box::new(err) as Box<dyn StdError + Send + Sync>),
            )?;

        let builder = artist::Entity::find().filter(without_trashed::<artist::Entity>());

        index_document::<artist::Entity, ArtistDocument, _>(
            &database,
            &typesense,
            builder,
            build_artist_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
