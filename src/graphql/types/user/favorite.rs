use crate::{
    AppError,
    entities::user::favorite,
    graphql::{
        loaders::user::favorite::{
            favorite_entry::FavoriteEntryLoader, favorite_user::FavoriteUserLoader,
        },
        types::auth::user::User,
    },
};
use async_graphql::{Context, Error, Object, Result, dataloader::DataLoader};

use crate::graphql::types::content::entry::Entry;

/// Represents a favorite of a user.
pub struct Favorite {
    pub favoriteable_type: String,
    pub favoriteable_id: u64,
    pub user_id: u64,
}

#[Object]
impl Favorite {
    async fn entry(&self, ctx: &Context<'_>) -> Result<Option<Entry>> {
        if self.favoriteable_type != "entry" {
            return Ok(None);
        }

        let loader = ctx.data_unchecked::<DataLoader<FavoriteEntryLoader>>();

        Ok(loader.load_one(self.favoriteable_id).await?.map(Into::into))
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
            favoriteable_type: model.favoriteable_type,
            favoriteable_id: model.favoriteable_id,
            user_id: model.user_id,
        }
    }
}
