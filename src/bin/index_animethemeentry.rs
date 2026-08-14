use animethemes_graphql_rust::entities::content::animethemeentry;
use animethemes_graphql_rust::scopes::without_trashed;
use animethemes_graphql_rust::typesense::documents::animethemeentry_document::{
    AnimeThemeEntryDocument, build_animethemeentry_documents,
};
use animethemes_graphql_rust::typesense::index_document::index_document;
use anyhow::{Context, Result};

use animethemes_graphql_rust::db::connect;
use animethemes_graphql_rust::typesense::client::create_typesense_client;

use sea_orm::{EntityTrait, QueryFilter};
use typesense::prelude::Document;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database = connect().await;
    let typesense = create_typesense_client();

    let collection = typesense.collection::<AnimeThemeEntryDocument>();

    match collection.retrieve().await {
        Ok(_) => {
            collection
                .delete()
                .await
                .context("failed to delete existing Typesense collection")?;
        }
        Err(_) => {}
    }

    typesense
        .collections()
        .create(AnimeThemeEntryDocument::collection_schema())
        .await
        .context("failed to create Typesense collection")?;

    let builder =
        animethemeentry::Entity::find().filter(without_trashed::<animethemeentry::Entity>());

    index_document::<animethemeentry::Entity, AnimeThemeEntryDocument, _>(
        &database,
        &typesense,
        builder,
        build_animethemeentry_documents,
    )
    .await
    .unwrap();

    Ok(())
}
