use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "crate::enums::content::resourcesite::ResourceSite")]
pub enum ResourceSite {
    OfficialSite,
    X,
    Anidb,
    Anilist,
    AnimePlanet,
    ANN,
    Kitsu,
    MAL,
    Wiki,
    Spotify,
    YoutubeMusic,
    Youtube,
    AppleMusic,
    AmazonMusic,
    Crunchyroll,
    Hidive,
    Netflix,
    DisneyPlus,
    Hulu,
    AmazonPrimeVideo,
    Livechart,
}
