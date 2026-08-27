use loco_rs::prelude::*;
use sea_orm::{EntityTrait, QueryFilter};
use std::error::Error as StdError;
use typesense::prelude::Document;

use crate::{
    entities::content::series,
    scopes::without_trashed,
    typesense::{
        client::TypesenseClient,
        documents::series_document::{SeriesDocument, build_series_documents},
        index_document,
    },
};

pub struct SearchIndexSeries;

#[async_trait]
impl Task for SearchIndexSeries {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index-series".to_string(),
            detail: "Import series from the database into the search index".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let typesense = app_context
            .shared_store
            .get::<TypesenseClient>()
            .expect("Typesense not initialized");

        let collection = typesense.collection::<SeriesDocument>();

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
            .create(SeriesDocument::collection_schema())
            .await
            .map_err(
                |err| loco_rs::Error::from(Box::new(err) as Box<dyn StdError + Send + Sync>),
            )?;

        let builder = series::Entity::find().filter(without_trashed::<series::Entity>());

        index_document::<series::Entity, SeriesDocument, _>(
            &app_context.db,
            &typesense,
            builder,
            build_series_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
