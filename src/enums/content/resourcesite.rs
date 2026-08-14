use sea_orm::entity::prelude::*;

use crate::enums::LocalizedEnum;

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ResourceSite {
    #[sea_orm(num_value = 0)]
    OfficialSite,

    #[sea_orm(num_value = 1)]
    X,

    #[sea_orm(num_value = 2)]
    Anidb,

    #[sea_orm(num_value = 3)]
    Anilist,

    #[sea_orm(num_value = 4)]
    AnimePlanet,

    #[sea_orm(num_value = 5)]
    ANN,

    #[sea_orm(num_value = 6)]
    Kitsu,

    #[sea_orm(num_value = 7)]
    MAL,

    #[sea_orm(num_value = 8)]
    Wiki,

    #[sea_orm(num_value = 9)]
    Spotify,

    #[sea_orm(num_value = 10)]
    YoutubeMusic,

    #[sea_orm(num_value = 11)]
    Youtube,

    #[sea_orm(num_value = 12)]
    AppleMusic,

    #[sea_orm(num_value = 13)]
    AmazonMusic,

    #[sea_orm(num_value = 14)]
    Crunchyroll,

    #[sea_orm(num_value = 15)]
    Hidive,

    #[sea_orm(num_value = 16)]
    Netflix,

    #[sea_orm(num_value = 17)]
    DisneyPlus,

    #[sea_orm(num_value = 18)]
    Hulu,

    #[sea_orm(num_value = 19)]
    AmazonPrimeVideo,

    #[sea_orm(num_value = 20)]
    Livechart,
}

impl LocalizedEnum for ResourceSite {
    fn localize(&self) -> &str {
        match self {
            ResourceSite::OfficialSite => "Official Website",
            ResourceSite::X => "X",
            ResourceSite::Anidb => "aniDB",
            ResourceSite::Anilist => "AniList",
            ResourceSite::AnimePlanet => "Anime-Planet",
            ResourceSite::ANN => "Anime News Network",
            ResourceSite::Kitsu => "Kitsu",
            ResourceSite::MAL => "MyAnimeList",
            ResourceSite::Wiki => "Wiki",
            ResourceSite::Spotify => "Spotify",
            ResourceSite::YoutubeMusic => "Youtube Music",
            ResourceSite::Youtube => "Youtube",
            ResourceSite::AppleMusic => "Apple Music",
            ResourceSite::AmazonMusic => "Amazon Music",
            ResourceSite::Crunchyroll => "Crunchyroll",
            ResourceSite::Hidive => "HIDIVE",
            ResourceSite::Netflix => "Netflix",
            ResourceSite::DisneyPlus => "Disney Plus",
            ResourceSite::Hulu => "Hulu",
            ResourceSite::AmazonPrimeVideo => "Amazon Prime Video",
            ResourceSite::Livechart => "Livechart",
        }
    }
}
