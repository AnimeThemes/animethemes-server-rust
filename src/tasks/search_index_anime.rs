use loco_rs::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error as StdError;
use typesense::prelude::Document;

use crate::{
    entities::content::anime,
    scopes::without_trashed,
    typesense::{
        client::TypesenseClient,
        documents::anime_document::{AnimeDocument, build_anime_documents},
        index_document,
    },
};

pub struct SearchIndexAnime;

#[async_trait]
impl Task for SearchIndexAnime {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index-anime".to_string(),
            detail: "Import anime from the database into the search index".to_string(),
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

        let collection = typesense.collection::<AnimeDocument>();

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
            .create(AnimeDocument::collection_schema())
            .await
            .map_err(
                |err| loco_rs::Error::from(Box::new(err) as Box<dyn StdError + Send + Sync>),
            )?;

        let builder = anime::Entity::find().filter(without_trashed::<anime::Entity>());

        index_document::<anime::Entity, AnimeDocument, _>(
            &database,
            &typesense,
            builder,
            build_anime_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
