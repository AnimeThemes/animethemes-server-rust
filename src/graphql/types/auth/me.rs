use async_graphql::{
    ComplexObject, Context, InputObject, Result, SimpleObject, dataloader::DataLoader,
};
use chrono::{DateTime, Utc};

use crate::{
    entities::auth::user,
    graphql::{
        enums::sort::list::playlist_sort::PlaylistSort,
        loaders::auth::user::{
            user_favorites::{
                UserFavoritesLoader, UserFavoritesLoaderKey, UserFavoritesLoaderQuery,
            },
            user_playlists::{UserPlaylistsLoader, UserPlaylistsLoaderKey},
            user_roles::UserRolesLoader,
            user_watchhistory::UserWatchHistoryLoader,
        },
        types::{
            auth::role::Role,
            list::playlist::Playlist,
            user::{favorite::Favorite, watchhistory::WatchHistory},
        },
    },
};

#[derive(InputObject, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserFavoritesFilterInput {
    pub entry_id: Option<u64>,
}

/// Represents an AnimeThemes account.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Me {
    /// The primary key of the resource
    pub id: u64,
    /// The username of the resource
    pub name: String,
    /// The email of the user
    pub email: String,
    /// The date the user verified their email
    pub email_verified_at: Option<DateTime<Utc>>,
    /// The date that the resource was created
    pub created_at: DateTime<Utc>,
    /// The date that the resource was updated
    pub updated_at: DateTime<Utc>,
}

impl From<user::Model> for Me {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            email: model.email,
            email_verified_at: model.email_verified_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[ComplexObject]
impl Me {
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

    async fn roles(&self, ctx: &Context<'_>) -> Result<Vec<Role>> {
        let loader = ctx.data_unchecked::<DataLoader<UserRolesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Role::from).collect())
    }

    async fn watch_history(&self, ctx: &Context<'_>) -> Result<Vec<WatchHistory>> {
        let loader = ctx.data_unchecked::<DataLoader<UserWatchHistoryLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(WatchHistory::from).collect())
    }

    async fn favorites(
        &self,
        ctx: &Context<'_>,
        filter: Option<UserFavoritesFilterInput>,
    ) -> Result<Vec<Favorite>> {
        let loader = ctx.data_unchecked::<DataLoader<UserFavoritesLoader>>();

        let models = loader
            .load_one(UserFavoritesLoaderKey {
                key: self.id,
                query: UserFavoritesLoaderQuery { filter },
            })
            .await?
            .unwrap_or_default();

        Ok(models.into_iter().map(Favorite::from).collect())
    }
}
