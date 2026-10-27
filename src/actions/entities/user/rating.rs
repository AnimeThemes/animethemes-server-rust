use crate::AppError;
use crate::entities::content::entry;
use crate::entities::user::rating;
use crate::rules::validation_error::ValidationError;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, SelectExt};
use sea_query::OnConflict;

pub struct RateEntryActionParameters {
    pub entry_id: u64,
    pub user_id: u64,
    pub score: Option<f32>,
}

pub struct RatingAction;

impl RatingAction {
    async fn validate(
        db: &DatabaseConnection,
        params: &RateEntryActionParameters,
    ) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let entry_exists = entry::Entity::find()
            .filter(entry::Column::Id.eq(params.entry_id))
            .exists(db)
            .await?;

        if !entry_exists {
            errors.push(ValidationError::new(
                "entry_id",
                vec!["The entry does not exist."],
            ));
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn rate_entry(
        db: &DatabaseConnection,
        params: RateEntryActionParameters,
    ) -> Result<Option<rating::Model>, AppError> {
        Self::validate(db, &params).await?;

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
                    .exec_with_returning(db)
                    .await?;

                Ok(Some(model))
            }

            None => {
                rating::Entity::delete_many()
                    .filter(rating::Column::UserId.eq(params.user_id))
                    .filter(rating::Column::EntryId.eq(params.entry_id))
                    .exec(db)
                    .await?;

                Ok(None)
            }
        }
    }

    pub async fn delete_all(db: &DatabaseConnection, user_id: u64) -> Result<(), AppError> {
        rating::Entity::delete_many()
            .filter(rating::Column::UserId.eq(user_id))
            .exec(db)
            .await?;

        Ok(())
    }
}
