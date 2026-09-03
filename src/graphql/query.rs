use async_graphql::{Context, MergedObject, Object, Result};

use crate::{
    graphql::{
        queries::{
            admin::{announcement::AnnouncementQuery, featuredtheme::FeaturedThemeQuery},
            content::{
                anime::AnimeQuery, animeyear::AnimeYearQuery, artist::ArtistQuery,
                entry::EntryQuery, image::ImageQuery, series::SeriesQuery, studio::StudioQuery,
                theme::ThemeQuery, video::VideoQuery,
            },
            document::page::PageQuery,
            list::playlist::PlaylistQuery,
            search::SearchQuery,
        },
        types::auth::{me::Me, permissions::Permissions},
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
    ThemeQuery,
    EntryQuery,
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
        let Some(user) = ctx.data_opt::<CurrentUser>() else {
            return Ok(None);
        };

        Ok(Some(Me::from(user.user.clone())))
    }

    async fn permissions(&self, _ctx: &Context<'_>) -> Permissions {
        Permissions
    }
}
