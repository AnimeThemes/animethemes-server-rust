use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "crate::enums::content::videooverlap::VideoOverlap")]
pub enum VideoOverlap {
    None,
    Trans,
    Over,
}
