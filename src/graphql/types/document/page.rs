use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};
use chrono::{DateTime, Utc};

use crate::{
    entities::document::page,
    graphql::{loaders::document::page_page::PagePageLoader, utils::format_datetime},
};

/// Represents a static markdown page used for guides and other documentation.
///
/// For example, the 'encoding/audio_normalization' page represents the documentation for audio normalization.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Page {
    /// The primary key of the resource
    pub id: u64,
    /// The primary title of the page
    pub name: String,
    /// The URL slug & route key of the resource
    pub slug: String,
    /// The body content of the resource
    pub body: String,
    #[graphql(skip)]
    pub previous_id: Option<u64>,
    #[graphql(skip)]
    pub next_id: Option<u64>,
    #[graphql(skip)]
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl Page {
    /// The date that the resource was created
    async fn created_at(&self, #[graphql(default = "%+")] format: String) -> String {
        format_datetime(&self.created_at, &format)
    }

    /// The date that the resource was updated
    async fn updated_at(&self, #[graphql(default = "%+")] format: String) -> String {
        format_datetime(&self.updated_at, &format)
    }

    async fn previous(&self, ctx: &Context<'_>) -> Result<Option<Page>> {
        let Some(previous_id) = self.previous_id else {
            return Ok(None);
        };

        let loader = ctx.data_unchecked::<DataLoader<PagePageLoader>>();

        Ok(loader.load_one(previous_id).await?.map(Into::into))
    }

    async fn next(&self, ctx: &Context<'_>) -> Result<Option<Page>> {
        let Some(next_id) = self.next_id else {
            return Ok(None);
        };

        let loader = ctx.data_unchecked::<DataLoader<PagePageLoader>>();

        Ok(loader.load_one(next_id).await?.map(Into::into))
    }
}

impl From<page::Model> for Page {
    fn from(model: page::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
            body: model.body,
            previous_id: model.previous_id,
            next_id: model.next_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
