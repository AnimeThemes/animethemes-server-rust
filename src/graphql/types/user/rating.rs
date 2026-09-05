use crate::{
    AppError, entities::user::rating,
    graphql::loaders::user::rating::rating_entry::RatingEntryLoader,
    graphql::types::content::entry::Entry,
};
use async_graphql::{ComplexObject, Context, Error, Result, SimpleObject, dataloader::DataLoader};

/// Represents the rating of the authenticated user.
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Rating {
    #[graphql(skip)]
    pub entry_id: u64,
    #[graphql(skip)]
    pub user_id: u64,
    /// The score of the rating.
    pub score: f32,
}

#[ComplexObject]
impl Rating {
    async fn entry(&self, ctx: &Context<'_>) -> Result<Entry> {
        let loader = ctx.data_unchecked::<DataLoader<RatingEntryLoader>>();

        Ok(loader
            .load_one(self.entry_id)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?
            .into())
    }
}

impl From<rating::Model> for Rating {
    fn from(model: rating::Model) -> Self {
        Self {
            entry_id: model.entry_id,
            user_id: model.user_id,
            score: model.score,
        }
    }
}
