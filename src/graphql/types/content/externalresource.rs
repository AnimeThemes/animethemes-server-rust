use animethemes_server_rust::enums::LocalizedEnum;
use async_graphql::SimpleObject;

use crate::{
    entities::content::externalresource::{self},
    graphql::enums::content::resourcesite::ResourceSite,
};

/// Represents a site with supplementary information for another resource such as an anime or artist.
///
/// For example, the Bakemonogatari anime has MyAnimeList, AniList and AniDB resources.
#[derive(SimpleObject)]
pub struct ExternalResource {
    /// The primary key of the resource
    pub id: u64,
    /// The primary key of the resource in the external site
    pub external_id: Option<i32>,
    /// The URL of the external site
    pub link: String,
    /// The external site that the resource belongs to
    pub site: ResourceSite,
    /// The localized string value of the site field
    pub site_localized: String,
}

impl From<externalresource::Model> for ExternalResource {
    fn from(model: externalresource::Model) -> Self {
        Self {
            id: model.id,
            external_id: model.external_id,
            link: model.link,
            site: model.site.into(),
            site_localized: model.site.localize().to_string(),
        }
    }
}
