use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "crate::enums::content::imagefacet::ImageFacet")]
pub enum ImageFacet {
    SmallCover,
    LargeCover,
    Grill,
    Document,
    Avatar,
    Banner,
}
