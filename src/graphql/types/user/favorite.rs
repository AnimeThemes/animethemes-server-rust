use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Error, Result, SimpleObject, Union};

use crate::AppError;
use crate::entities::user::favorite;
use crate::graphql::loaders::user::favorite::favorite_entry::FavoriteEntryLoader;
use crate::graphql::loaders::user::favorite::favorite_user::FavoriteUserLoader;
use crate::graphql::types::auth::user::User;
use crate::graphql::types::content::entry::Entry;

#[derive(Union)]
enum Favoriteable {
    Entry(Entry),
}

/// Represents a favorite of a user.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Favorite {
    pub id: u64,
    pub position: i32,
    #[graphql(skip)]
    pub favoriteable_type: favorite::FavoriteableType,
    #[graphql(skip)]
    pub favoriteable_id: u64,
    #[graphql(skip)]
    pub user_id: u64,
}

#[ComplexObject]
impl Favorite {
    async fn entry(&self, ctx: &Context<'_>) -> Result<Option<Entry>> {
        if self.favoriteable_type != favorite::FavoriteableType::Entry {
            return Ok(None);
        }

        let loader = ctx.data_unchecked::<DataLoader<FavoriteEntryLoader>>();

        Ok(loader.load_one(self.favoriteable_id).await?.map(Into::into))
    }

    async fn favoriteable(&self, ctx: &Context<'_>) -> Result<Favoriteable> {
        match self.favoriteable_type {
            favorite::FavoriteableType::Entry => {
                let loader = ctx.data_unchecked::<DataLoader<FavoriteEntryLoader>>();

                let entry = loader
                    .load_one(self.favoriteable_id)
                    .await?
                    .ok_or(AppError::NotFound)?;

                Ok(Favoriteable::Entry(entry.into()))
            }
        }
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<User> {
        let loader = ctx.data_unchecked::<DataLoader<FavoriteUserLoader>>();

        Ok(loader
            .load_one(self.user_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?
            .into())
    }
}

impl From<favorite::Model> for Favorite {
    fn from(model: favorite::Model) -> Self {
        Self {
            id: model.id,
            position: model.position,
            favoriteable_type: model.favoriteable_type,
            favoriteable_id: model.favoriteable_id,
            user_id: model.user_id,
        }
    }
}
