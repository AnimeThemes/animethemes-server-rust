use async_graphql::{Context, MergedObject, Object, Result};

use crate::{
    graphql::{
        queries::{
            admin::{announcement::AnnouncementQuery, featuredtheme::FeaturedThemeQuery},
            content::{
                anime::AnimeQuery, animetheme::AnimeThemeQuery,
                animethemeentry::AnimeThemeEntryQuery, animeyear::AnimeYearQuery,
                artist::ArtistQuery, image::ImageQuery, series::SeriesQuery, studio::StudioQuery,
                video::VideoQuery,
            },
            document::page::PageQuery,
            list::playlist::PlaylistQuery,
            search::SearchQuery,
        },
        types::auth::me::Me,
    },
    middlewares::current_user::CurrentUser,
};

#[derive(MergedObject, Default)]
pub struct Query(
    RootQuery,
    SearchQuery,
    AnnouncementQuery,
    FeaturedThemeQuery,
    PageQuery,
    PlaylistQuery,
    AnimeQuery,
    AnimeYearQuery,
    AnimeThemeQuery,
    AnimeThemeEntryQuery,
    ArtistQuery,
    ImageQuery,
    SeriesQuery,
    StudioQuery,
    VideoQuery,
);

#[derive(Default)]
struct RootQuery;

#[Object]
impl RootQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<Me>> {
        let user = ctx.data::<CurrentUser>().ok();

        if let Some(user) = user {
            return Ok(Some(Me::from(&user.user)));
        }

        Ok(None)
    }
}
