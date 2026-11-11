use async_graphql::{Context, MergedObject, Object, Result};

use crate::graphql::queries::admin::announcement::AnnouncementQuery;
use crate::graphql::queries::admin::featuredtheme::FeaturedThemeQuery;
use crate::graphql::queries::content::anime::AnimeQuery;
use crate::graphql::queries::content::animeyear::AnimeYearQuery;
use crate::graphql::queries::content::artist::ArtistQuery;
use crate::graphql::queries::content::entry::EntryQuery;
use crate::graphql::queries::content::image::ImageQuery;
use crate::graphql::queries::content::series::SeriesQuery;
use crate::graphql::queries::content::studio::StudioQuery;
use crate::graphql::queries::content::theme::ThemeQuery;
use crate::graphql::queries::content::video::VideoQuery;
use crate::graphql::queries::document::page::PageQuery;
use crate::graphql::queries::list::playlist::PlaylistQuery;
use crate::graphql::queries::search::SearchQuery;
use crate::graphql::types::auth::me::Me;
use crate::graphql::types::auth::permissions::Permissions;
use crate::middlewares::current_user::CurrentUser;

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
