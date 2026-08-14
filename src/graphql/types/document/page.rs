use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};

use crate::{entities::document::page, graphql::loaders::document::page_page::PagePageLoader};

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
}

#[ComplexObject]
impl Page {
    async fn previous(&self, ctx: &Context<'_>) -> Result<Option<Page>> {
        let Some(previous_id) = self.previous_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<PagePageLoader>>()?;

        Ok(loader.load_one(previous_id).await?.map(Into::into))
    }

    async fn next(&self, ctx: &Context<'_>) -> Result<Option<Page>> {
        let Some(next_id) = self.next_id else {
            return Ok(None);
        };

        let loader = ctx.data::<DataLoader<PagePageLoader>>()?;

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
        }
    }
}
