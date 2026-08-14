use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "crate::enums::content::animeseason::AnimeSeason")]
pub enum AnimeSeason {
    Winter,
    Spring,
    Summer,
    Fall,
}
