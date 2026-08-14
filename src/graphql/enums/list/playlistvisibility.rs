use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "crate::enums::list::playlistvisibility::PlaylistVisibility")]
pub enum PlaylistVisibility {
    Public,
    Private,
    Unlisted,
}
