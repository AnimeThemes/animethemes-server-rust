use animethemes_server_rust::entities::content::artist;
use animethemes_server_rust::scopes::without_trashed;

use animethemes_server_rust::typesense::documents::artist_document::{
    ArtistDocument, build_artist_documents,
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

    let collection = typesense.collection::<ArtistDocument>();

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
        .create(ArtistDocument::collection_schema())
        .await
        .context("failed to create Typesense collection")?;

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
