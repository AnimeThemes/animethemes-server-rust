use loco_rs::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error as StdError;
use typesense::prelude::Document;

use crate::{
    entities::content::animetheme,
    scopes::without_trashed,
    typesense::{
        client::TypesenseClient,
        documents::animetheme_document::{AnimeThemeDocument, build_animetheme_documents},
        index_document,
    },
};

pub struct SearchIndexAnimeTheme;

#[async_trait]
impl Task for SearchIndexAnimeTheme {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index_animetheme".to_string(),
            detail: "Import animethemes from the database into the search index".to_string(),
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

        let collection = typesense.collection::<AnimeThemeDocument>();

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
            .create(AnimeThemeDocument::collection_schema())
            .await
            .map_err(
                |err| loco_rs::Error::from(Box::new(err) as Box<dyn StdError + Send + Sync>),
            )?;

        let builder = animetheme::Entity::find().filter(without_trashed::<animetheme::Entity>());

        index_document::<animetheme::Entity, AnimeThemeDocument, _>(
            &database,
            &typesense,
            builder,
            build_animetheme_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
