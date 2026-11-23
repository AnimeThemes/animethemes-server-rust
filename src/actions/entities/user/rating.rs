use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, SelectExt};
use sea_query::OnConflict;
use validator::{ValidationError, ValidationErrors};

use crate::AppError;
use crate::entities::content::entry;
use crate::entities::user::rating;

#[derive(InputObject)]
#[graphql(name = "RateEntryInput")]
pub struct RateEntryActionParameters {
    #[graphql(skip)]
    pub entry_id: u64,

    #[graphql(skip)]
    pub user_id: u64,

    /// The score to rate the entry with.
    pub score: Option<f32>,
}

pub struct RatingAction;

impl RatingAction {
    async fn validate(
        db: &DatabaseConnection,
        params: &RateEntryActionParameters,
    ) -> Result<(), AppError> {
        let exists = entry::Entity::find_by_id(params.entry_id)
            .exists(db)
            .await?;

        if !exists {
            let mut errors = ValidationErrors::new();

            errors.add(
                "entry_id",
                ValidationError::new("not_found").with_message("The entry does not exist.".into()),
            );

            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn rate_entry(
        ctx: &AppContext,
        params: RateEntryActionParameters,
    ) -> Result<Option<rating::Model>, AppError> {
        Self::validate(&ctx.db, &params).await?;

        match params.score {
            Some(score) => {
                let model = rating::ActiveModel {
                    entry_id: Set(params.entry_id),
                    user_id: Set(params.user_id),
                    score: Set(score),
                    ..Default::default()
                };

                let model = rating::Entity::insert(model)
                    .on_conflict(
                        OnConflict::columns([rating::Column::EntryId, rating::Column::UserId])
                            .update_column(rating::Column::Score)
                            .to_owned(),
                    )
                    .exec_with_returning(&ctx.db)
                    .await?;

                Ok(Some(model))
            }

            None => {
                rating::Entity::delete_many()
                    .filter(rating::Column::UserId.eq(params.user_id))
                    .filter(rating::Column::EntryId.eq(params.entry_id))
                    .exec(&ctx.db)
                    .await?;

                Ok(None)
            }
        }
    }

    pub async fn delete_all(ctx: &AppContext, user_id: u64) -> Result<(), AppError> {
        rating::Entity::delete_many()
            .filter(rating::Column::UserId.eq(user_id))
            .exec(&ctx.db)
            .await?;

        Ok(())
    }
}
