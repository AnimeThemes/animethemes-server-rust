use async_graphql::connection::{Connection, EmptyFields, OpaqueCursor};
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use chrono::{DateTime, Utc};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter};

use crate::entities::auth::user;
use crate::entities::list::playlist;
use crate::entities::user::watchhistory;
use crate::graphql::cursor::{CursorSort, CursorSortKey, PaginationCursor, cursor_paginate};
use crate::graphql::enums::sort::GraphQLSort;
use crate::graphql::enums::sort::list::playlist_sort::PlaylistSort;
use crate::graphql::enums::sort::user::favorite_sort::FavoriteSort;
use crate::graphql::enums::sort::user::rating_sort::RatingSort;
use crate::graphql::enums::sort::user::watchhistory_sort::WatchHistorySort;
use crate::graphql::inputs::pagination_input::PaginationInput;
use crate::graphql::loaders::auth::user::user_favorites::{
    UserFavoritesLoader, UserFavoritesLoaderKey, UserFavoritesLoaderQuery,
};
use crate::graphql::loaders::auth::user::user_ratings::{
    UserRatingsLoader, UserRatingsLoaderKey, UserRatingsLoaderQuery,
};
use crate::graphql::loaders::auth::user::user_roles::UserRolesLoader;
use crate::graphql::types::auth::role::Role;
use crate::graphql::types::list::playlist::Playlist;
use crate::graphql::types::user::favorite::Favorite;
use crate::graphql::types::user::rating::Rating;
use crate::graphql::types::user::watchhistory::WatchHistory;

#[derive(InputObject, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserFavoritesFilterInput {
    pub entry_id: Option<u64>,
}

/// Represents an Themes account.
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
    /// The playlists of the authenticated user.
    async fn playlists(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        sort: Option<Vec<PlaylistSort>>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, Playlist, EmptyFields, EmptyFields>>
    {
        let query = playlist::Entity::find().filter(playlist::Column::UserId.eq(self.id));

        let mut cursor_sorts = sort
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(PlaylistSort::cursor_sort)
            .collect::<Vec<_>>();

        if !cursor_sorts
            .iter()
            .any(|sort| matches!(&sort.key, CursorSortKey::Column(playlist::Column::Id)))
        {
            cursor_sorts.push(CursorSort {
                key: CursorSortKey::Column(playlist::Column::Id),
                order: Order::Asc,
            });
        }

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }

    /// The roles of the authenticated user.
    async fn roles(&self, ctx: &Context<'_>) -> Result<Vec<Role>> {
        let loader = ctx.data_unchecked::<DataLoader<UserRolesLoader>>();

        let models = loader.load_one(self.id).await?.unwrap_or_default();

        Ok(models.into_iter().map(Role::from).collect())
    }

    /// The watch history of the authenticated user.
    async fn watch_history(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
        sort: Option<Vec<WatchHistorySort>>,
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, WatchHistory, EmptyFields, EmptyFields>>
    {
        let query = watchhistory::Entity::find().filter(watchhistory::Column::UserId.eq(self.id));

        let mut cursor_sorts = sort
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(WatchHistorySort::cursor_sort)
            .collect::<Vec<_>>();

        if !cursor_sorts
            .iter()
            .any(|sort| matches!(&sort.key, CursorSortKey::Column(watchhistory::Column::Id)))
        {
            cursor_sorts.push(CursorSort {
                key: CursorSortKey::Column(watchhistory::Column::Id),
                order: Order::Asc,
            });
        }

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }

    /// The favorites of the authenticated user.
    async fn favorites(
        &self,
        ctx: &Context<'_>,
        filter: Option<UserFavoritesFilterInput>,
        sort: Option<Vec<FavoriteSort>>,
    ) -> Result<Vec<Favorite>> {
        let loader = ctx.data_unchecked::<DataLoader<UserFavoritesLoader>>();

        let models = loader
            .load_one(UserFavoritesLoaderKey {
                key: self.id,
                query: UserFavoritesLoaderQuery { filter, sort },
            })
            .await?
            .unwrap_or_default();

        Ok(models.into_iter().map(Favorite::from).collect())
    }

    /// The ratings of the authenticated user.
    async fn ratings(
        &self,
        ctx: &Context<'_>,
        sort: Option<Vec<RatingSort>>,
    ) -> Result<Vec<Rating>> {
        let loader = ctx.data_unchecked::<DataLoader<UserRatingsLoader>>();

        let models = loader
            .load_one(UserRatingsLoaderKey {
                key: self.id,
                query: UserRatingsLoaderQuery { sort },
            })
            .await?
            .unwrap_or_default();

        Ok(models.into_iter().map(Rating::from).collect())
    }
}
