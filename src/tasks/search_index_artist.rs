use std::error::Error as StdError;

use loco_rs::prelude::*;
use sea_orm::{EntityTrait, QueryFilter};
use typesense::prelude::Document;

use crate::entities::content::artist;
use crate::scopes::without_trashed;
use crate::typesense::client::TypesenseClient;
use crate::typesense::documents::artist_document::{ArtistDocument, build_artist_documents};
use crate::typesense::index_document;

pub struct SearchIndexArtist;

#[async_trait]
impl Task for SearchIndexArtist {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "search:index-artist".to_string(),
            detail: "Import artists from the database into the search index".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
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
            &app_context.db,
            &typesense,
            builder,
            build_artist_documents,
        )
        .await
        .unwrap();

        Ok(())
    }
}
