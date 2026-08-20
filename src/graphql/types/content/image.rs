use animethemes_server_rust::enums::{LocalizedEnum, content::imagefacet::ImageFacet};
use async_graphql::SimpleObject;

use crate::entities::content::image::{self};

/// Represents a visual component for another resource such as an anime or artist.
///
/// For example, the Bakemonogatari anime has two images to represent small and large cover images.
#[derive(SimpleObject)]
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
}

impl From<image::Model> for Image {
    fn from(model: image::Model) -> Self {
        Self {
            id: model.id,
            facet: model.facet,
            facet_localized: model.facet.localize().to_string(),
            path: model.path.clone(),
            link: model.link(),
        }
    }
}
