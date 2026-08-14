use async_graphql::MergedObject;

use crate::graphql::mutations::{
    auth::AuthMutation,
    list::{playlist::PlaylistMutation, track::PlaylistTrackMutation},
};

#[derive(MergedObject, Default)]
pub struct Mutation(AuthMutation, PlaylistMutation, PlaylistTrackMutation);
