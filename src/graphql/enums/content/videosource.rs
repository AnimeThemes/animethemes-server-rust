use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "crate::enums::content::videosource::VideoSource")]
pub enum VideoSource {
    WEB,
    RAW,
    BD,
    DVD,
    VHS,
    LD,
}
