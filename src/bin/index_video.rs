use animethemes_graphql_rust::entities::content::video;
use animethemes_graphql_rust::scopes::without_trashed;
use animethemes_graphql_rust::typesense::documents::video_document::{
    VideoDocument, build_video_documents,
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

    let collection = typesense.collection::<VideoDocument>();

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
        .create(VideoDocument::collection_schema())
        .await
        .context("failed to create Typesense collection")?;

    let builder = video::Entity::find().filter(without_trashed::<video::Entity>());

    index_document::<video::Entity, VideoDocument, _>(
        &database,
        &typesense,
        builder,
        build_video_documents,
    )
    .await
    .unwrap();

    Ok(())
}
