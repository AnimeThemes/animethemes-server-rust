use async_graphql::MergedObject;

use crate::graphql::mutations::{
    auth::AuthMutation,
    list::{playlist::PlaylistMutation, track::PlaylistTrackMutation},
    user::{like::LikeMutation, watch::WatchMutation},
};

#[derive(MergedObject, Default)]
pub struct Mutation(
    AuthMutation,
    PlaylistMutation,
    PlaylistTrackMutation,
    LikeMutation,
    WatchMutation,
);
