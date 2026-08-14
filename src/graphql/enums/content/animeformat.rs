use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "crate::enums::content::animeformat::AnimeFormat")]
pub enum AnimeFormat {
    TV,
    TVShort,
    OVA,
    Movie,
    Special,
    ONA,
}
