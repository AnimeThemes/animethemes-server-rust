use async_graphql::{ComplexObject, Context, Result, SimpleObject, dataloader::DataLoader};

use crate::{
    entities::auth::user,
    graphql::{
        loaders::auth::user::user_playlists::UserPlaylistsLoader, types::list::playlist::Playlist,
    },
};

/// Represents an AnimeThemes account.
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
    async fn playlists(&self, ctx: &Context<'_>) -> Result<Vec<Playlist>> {
        let loader = ctx.data::<DataLoader<UserPlaylistsLoader>>()?;

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Playlist::from).collect())
    }
}
