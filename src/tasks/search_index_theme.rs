use std::error::Error as StdError;

use loco_rs::prelude::*;
use sea_orm::{EntityTrait, QueryFilter};
use typesense::prelude::Document;

use crate::entities::content::theme;
use crate::scopes::without_trashed;
use crate::typesense::client::TypesenseClient;
use crate::typesense::documents::theme_document::{ThemeDocument, build_theme_documents};
use crate::typesense::index_document;

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
