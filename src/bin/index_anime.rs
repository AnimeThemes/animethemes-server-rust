use animethemes_server_rust::entities::content::anime;
use animethemes_server_rust::scopes::without_trashed;
use animethemes_server_rust::typesense::documents::anime_document::{
    AnimeDocument, build_anime_documents,
};
use animethemes_server_rust::typesense::index_document::index_document;
use anyhow::{Context, Result};

use animethemes_server_rust::db::connect;
use animethemes_server_rust::typesense::client::create_typesense_client;

use sea_orm::{EntityTrait, QueryFilter};
use typesense::prelude::Document;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database = connect().await;
    let typesense = create_typesense_client();

    let collection = typesense.collection::<AnimeDocument>();

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
        .create(AnimeDocument::collection_schema())
        .await
        .context("failed to create Typesense collection")?;

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
