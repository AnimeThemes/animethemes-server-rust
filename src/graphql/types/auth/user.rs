use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject};

use crate::entities::auth::user;
use crate::graphql::enums::sort::list::playlist_sort::PlaylistSort;
use crate::graphql::loaders::auth::user::user_playlists::{
    UserPlaylistsLoader, UserPlaylistsLoaderKey,
};
use crate::graphql::types::list::playlist::Playlist;

/// Represents an Themes account.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    /// The primary key of the resource
    pub id: u64,
    /// The username of the resource
    pub name: String,
}

impl From<user::Model> for User {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}

#[ComplexObject]
impl User {
    async fn playlists(
        &self,
        ctx: &Context<'_>,
        sort: Option<Vec<PlaylistSort>>,
    ) -> Result<Vec<Playlist>> {
        let loader = ctx.data_unchecked::<DataLoader<UserPlaylistsLoader>>();

        let models = loader
            .load_one(UserPlaylistsLoaderKey::new(self.id, sort))
            .await?
            .unwrap_or_default();

        Ok(models.into_iter().map(Playlist::from).collect())
    }
}
