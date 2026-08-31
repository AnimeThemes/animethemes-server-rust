use std::{future::Future, pin::Pin};

use anyhow::{Context, Result};
use sea_orm::{
    DatabaseConnection, EntityTrait, PaginatorTrait, Select, sea_query::value::prelude::serde_json,
};
use serde::{Serialize, de::DeserializeOwned};
use typesense::{
    models::{ImportDocumentsParameters, IndexAction},
    prelude::Document as TypesenseDocument,
};

use crate::typesense::client::TypesenseClient;

pub type BuildDocumentsFuture<'a, D> = Pin<Box<dyn Future<Output = Result<Vec<D>>> + Send + 'a>>;

pub async fn index_document<E, D, F>(
    database: &DatabaseConnection,
    typesense: &TypesenseClient,
    builder: Select<E>,
    mut build_documents: F,
) -> Result<()>
where
    E: EntityTrait,
    E::Model: Send + Sync + 'static,
    D: Serialize + DeserializeOwned + TypesenseDocument + Send + Sync + 'static,
    F: for<'a> FnMut(Vec<E::Model>, &'a DatabaseConnection) -> BuildDocumentsFuture<'a, D>,
{
    let paginator = builder.order_by_id_asc().paginate(database, 500);

    let total_pages = paginator
        .num_pages()
        .await
        .context("failed to count pages")?;

    let mut total_indexed = 0_u64;

    for page in 0..total_pages {
        let models = paginator.fetch_page(page).await.with_context(|| {
            format!(
                "failed to fetch page {page} for {}",
                E::table_name(&E::default())
            )
        })?;

        if models.is_empty() {
            continue;
        }

        let documents = build_documents(models, database).await.with_context(|| {
            format!(
                "failed to build documents for page {page} of {}",
                E::table_name(&E::default())
            )
        })?;

        if documents.is_empty() {
            continue;
        }

        let documents_count = documents.len() as u64;

        let jsonl = documents
            .iter()
            .map(serde_json::to_string)
            .collect::<serde_json::Result<Vec<_>>>()
            .context("failed to serialize documents as JSONL")?
            .join("\n");

        let parameters = ImportDocumentsParameters {
            action: Some(IndexAction::Upsert),
            return_id: Some(true),
            return_doc: Some(false),
            ..Default::default()
        };

        typesense
            .collection::<D>()
            .documents()
            .import_jsonl(jsonl, parameters)
            .await
            .with_context(|| {
                format!(
                    "failed to import page {page} of {} into Typesense",
                    E::table_name(&E::default())
                )
            })?;

        total_indexed += documents_count;

        tracing::info!(
            "{total_indexed} indexed documents for entity {}",
            E::table_name(&E::default())
        );
    }

    Ok(())
}
