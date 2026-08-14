use animethemes_server_rust::entities::content::animetheme;
use animethemes_server_rust::scopes::without_trashed;
use animethemes_server_rust::typesense::documents::animetheme_document::{
    AnimeThemeDocument, build_animetheme_documents,
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

    let collection = typesense.collection::<AnimeThemeDocument>();

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
        .create(AnimeThemeDocument::collection_schema())
        .await
        .context("failed to create Typesense collection")?;

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
