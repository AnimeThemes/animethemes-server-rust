use async_graphql::MergedObject;

use crate::graphql::mutations::auth::AuthMutation;
use crate::graphql::mutations::list::playlist::PlaylistMutation;
use crate::graphql::mutations::list::track::PlaylistTrackMutation;
use crate::graphql::mutations::user::favorite::FavoriteMutation;
use crate::graphql::mutations::user::watch::WatchMutation;

#[derive(MergedObject, Default)]
pub struct Mutation(
    AuthMutation,
    PlaylistMutation,
    PlaylistTrackMutation,
    FavoriteMutation,
    WatchMutation,
);
