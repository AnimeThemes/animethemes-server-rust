use loco_rs::prelude::*;
use sea_orm::{EntityTrait, QueryFilter};
use std::error::Error as StdError;
use typesense::prelude::Document;

use crate::{
    entities::content::theme,
    scopes::without_trashed,
    typesense::{
        client::TypesenseClient,
        documents::theme_document::{ThemeDocument, build_theme_documents},
        index_document,
    },
};

pub struct SearchIndexTheme;

#[async_trait]
impl Task for SearchIndexTheme {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index-theme".to_string(),
            detail: "Import themes from the database into the search index".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let typesense = app_context
            .shared_store
            .get::<TypesenseClient>()
            .expect("Typesense not initialized");

        let collection = typesense.collection::<ThemeDocument>();

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
            .create(ThemeDocument::collection_schema())
            .await
            .map_err(
                |err| loco_rs::Error::from(Box::new(err) as Box<dyn StdError + Send + Sync>),
            )?;

        let builder = theme::Entity::find().filter(without_trashed::<theme::Entity>());

        index_document::<theme::Entity, ThemeDocument, _>(
            &app_context.db,
            &typesense,
            builder,
            build_theme_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
