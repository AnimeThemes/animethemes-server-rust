use async_graphql::MergedObject;

use crate::graphql::mutations::admin::announcement::AnnouncementMutation;
use crate::graphql::mutations::admin::featuredtheme::FeaturedThemeMutation;
use crate::graphql::mutations::auth::AuthMutation;
use crate::graphql::mutations::content::anime::AnimeMutation;
use crate::graphql::mutations::content::artist::ArtistMutation;
use crate::graphql::mutations::content::entry::EntryMutation;
use crate::graphql::mutations::content::series::SeriesMutation;
use crate::graphql::mutations::content::song::SongMutation;
use crate::graphql::mutations::content::studio::StudioMutation;
use crate::graphql::mutations::content::theme::ThemeMutation;
use crate::graphql::mutations::list::playlist::PlaylistMutation;
use crate::graphql::mutations::list::track::PlaylistTrackMutation;
use crate::graphql::mutations::user::favorite::FavoriteMutation;
use crate::graphql::mutations::user::watch::WatchMutation;

#[derive(MergedObject, Default)]
pub struct Mutation(
    // Admin
    AnnouncementMutation,
    FeaturedThemeMutation,
    // Auth
    AuthMutation,
    // Content
    AnimeMutation,
    ArtistMutation,
    EntryMutation,
    SeriesMutation,
    SongMutation,
    StudioMutation,
    ThemeMutation,
    // List
    PlaylistMutation,
    PlaylistTrackMutation,
    // User
    FavoriteMutation,
    WatchMutation,
);
