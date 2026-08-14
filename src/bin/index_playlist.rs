use animethemes_graphql_rust::entities::list::playlist;
use animethemes_graphql_rust::scopes::list::playlist::public_playlists;
use animethemes_graphql_rust::typesense::documents::playlist_document::{
    PlaylistDocument, build_playlist_documents,
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

    let collection = typesense.collection::<PlaylistDocument>();

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
        .create(PlaylistDocument::collection_schema())
        .await
        .context("failed to create Typesense collection")?;

    let builder = playlist::Entity::find().filter(public_playlists());

    index_document::<playlist::Entity, PlaylistDocument, _>(
        &database,
        &typesense,
        builder,
        build_playlist_documents,
    )
    .await
    .unwrap();

    Ok(())
}
