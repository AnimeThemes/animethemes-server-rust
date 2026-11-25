use async_graphql::{ComplexObject, Result, SimpleObject};

use crate::entities::content::resource::{self};
use crate::enums::LocalizedEnum;
use crate::enums::content::resourcesite::ResourceSite;
use crate::graphql::utils::format_datetime;

/// Represents a site with supplementary information for another resource such as an anime or artist.
///
/// For example, the Bakemonogatari anime has MyAnimeList, AniList and AniDB resources.
#[derive(SimpleObject)]
#[graphql(complex)]
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
    /// Used to distinguish resources that map to the same resourceable
    pub r#as: Option<String>,

    #[graphql(skip)]
    pub created_at: time::OffsetDateTime,
    #[graphql(skip)]
    pub updated_at: time::OffsetDateTime,
}

#[ComplexObject]
impl ExternalResource {
    /// The date that the resource was created
    async fn created_at(
        &self,
        #[graphql(default = "%Y-%m-%d %H:%M:%S")] format: String,
    ) -> Result<String> {
        format_datetime(&self.created_at, &format)
    }

    async fn updated_at(
        &self,
        #[graphql(default = "%Y-%m-%d %H:%M:%S")] format: String,
    ) -> Result<String> {
        format_datetime(&self.updated_at, &format)
    }
}

impl From<resource::Model> for ExternalResource {
    fn from(model: resource::Model) -> Self {
        Self {
            id: model.id,
            external_id: model.external_id,
            link: model.link,
            site: model.site,
            site_localized: model.site.localize().to_string(),
            r#as: model.r#as,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
