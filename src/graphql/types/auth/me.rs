use async_graphql::{
    ComplexObject, Context, InputObject, Result, SimpleObject,
    connection::{Connection, EmptyFields, OpaqueCursor},
    dataloader::DataLoader,
};
use chrono::{DateTime, Utc};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter};

use crate::{
    entities::{auth::user, list::playlist, user::watchhistory},
    graphql::{
        cursor::{CursorSort, PaginationCursor, cursor_paginate},
        enums::sort::{
            GraphQLSort, list::playlist_sort::PlaylistSort, user::rating_sort::RatingSort,
        },
        inputs::pagination_input::PaginationInput,
        loaders::auth::user::{
            user_favorites::{
                UserFavoritesLoader, UserFavoritesLoaderKey, UserFavoritesLoaderQuery,
            },
            user_ratings::{UserRatingsLoader, UserRatingsLoaderKey, UserRatingsLoaderQuery},
            user_roles::UserRolesLoader,
        },
        types::{
            auth::role::Role,
            list::playlist::Playlist,
            user::{favorite::Favorite, rating::Rating, watchhistory::WatchHistory},
        },
    },
};

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
        let mut query = playlist::Entity::find().filter(playlist::Column::UserId.eq(self.id));

        if let Some(sorts) = sort.clone() {
            for sort in sorts {
                query = sort.apply_sort(query);
            }
        }

        let mut cursor_sorts = sort
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(PlaylistSort::cursor_sort)
            .collect::<Vec<_>>();

        cursor_sorts.push(CursorSort {
            column: playlist::Column::Id,
            order: Order::Asc,
        });

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
    ) -> Result<Connection<OpaqueCursor<PaginationCursor>, WatchHistory, EmptyFields, EmptyFields>>
    {
        let query = watchhistory::Entity::find().filter(watchhistory::Column::UserId.eq(self.id));

        let cursor_sorts = vec![CursorSort {
            column: watchhistory::Column::Id,
            order: Order::Asc,
        }];

        cursor_paginate(query, ctx, cursor_sorts, pagination).await
    }

    /// The favorites of the authenticated user.
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
