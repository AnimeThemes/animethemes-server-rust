use crate::{
    AppError,
    entities::user::like,
    graphql::{
        loaders::user::like::{like_entry::LikeEntryLoader, like_user::LikeUserLoader},
        types::auth::user::User,
    },
};
use async_graphql::{Context, Error, Object, Result, dataloader::DataLoader};

use crate::graphql::types::content::animethemeentry::AnimeThemeEntry;

/// Represents a like of a user.
pub struct Like {
    pub likeable_type: String,
    pub likeable_id: u64,
    pub user_id: u64,
}

#[Object]
impl Like {
    async fn animethemeentry(&self, ctx: &Context<'_>) -> Result<Option<AnimeThemeEntry>> {
        if self.likeable_type != "animethemeentry" {
            return Ok(None);
        }

        let loader = ctx.data_unchecked::<DataLoader<LikeEntryLoader>>();

        Ok(loader.load_one(self.likeable_id).await?.map(Into::into))
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<User> {
        let loader = ctx.data_unchecked::<DataLoader<LikeUserLoader>>();

        Ok(loader
            .load_one(self.user_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?
            .into())
    }
}

impl From<like::Model> for Like {
    fn from(model: like::Model) -> Self {
        Self {
            likeable_type: model.likeable_type,
            likeable_id: model.likeable_id,
            user_id: model.user_id,
        }
    }
}
