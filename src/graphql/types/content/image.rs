use async_graphql::{ComplexObject, SimpleObject};
use chrono::{DateTime, Utc};

use crate::entities::content::image::{self};
use crate::enums::LocalizedEnum;
use crate::enums::content::imagefacet::ImageFacet;
use crate::graphql::utils::format_datetime;

/// Represents a visual component for another resource such as an anime or artist.
///
/// For example, the Bakemonogatari anime has two images to represent small and large cover images.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Image {
    /// The primary key of the resource
    pub id: u64,
    /// The component that the resource is intended for
    pub facet: ImageFacet,
    /// The localized string value of the facet field
    pub facet_localized: String,
    /// The path of the file in storage
    pub path: String,
    /// The URL to stream the file from storage
    pub link: String,
    /// Used to sort the images scoped by their entity
    pub depth: i32,

    #[graphql(skip)]
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl Image {
    /// The date that the resource was created
    async fn created_at(&self, #[graphql(default = "%+")] format: String) -> String {
        format_datetime(&self.created_at, &format)
    }

    async fn updated_at(&self, #[graphql(default = "%+")] format: String) -> String {
        format_datetime(&self.updated_at, &format)
    }
}

impl From<image::Model> for Image {
    fn from(model: image::Model) -> Self {
        Self {
            id: model.id,
            facet: model.facet,
            facet_localized: model.facet.localize().to_string(),
            path: model.path.clone(),
            link: model.link(),
            depth: model.depth,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
