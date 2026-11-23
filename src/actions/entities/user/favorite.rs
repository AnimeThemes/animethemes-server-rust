use async_graphql::InputObject;
use loco_rs::app::AppContext;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
    QueryOrder, QuerySelect, SelectExt, TransactionTrait,
};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::AppError;
use crate::entities::content::entry;
use crate::entities::user::favorite;
use crate::traits::sortable::Sortable;

#[derive(Copy, Clone)]
pub enum FavoriteableType {
    Entry(u64),
}

impl From<FavoriteableType> for favorite::FavoriteableType {
    fn from(value: FavoriteableType) -> Self {
        match value {
            FavoriteableType::Entry(_) => favorite::FavoriteableType::Entry,
        }
    }
}

pub struct ToggleFavoriteActionParameters {
    pub favoriteable: FavoriteableType,
    pub user_id: u64,
}

#[derive(InputObject, Validate)]
#[graphql(name = "UpdateFavoriteInput")]
pub struct UpdateFavoriteActionParameters {
    #[validate(range(min = 1, message = "The position must be greater than 0."))]
    pub position: Option<i32>,
}

pub struct FavoriteAction;

impl FavoriteAction {
    async fn validate(
        db: &DatabaseConnection,
        params: &ToggleFavoriteActionParameters,
    ) -> Result<(), AppError> {
        let exists = match &params.favoriteable {
            FavoriteableType::Entry(id) => {
                entry::Entity::find()
                    .filter(entry::Column::Id.eq(*id))
                    .exists(db)
                    .await?
            }
        };

        if !exists {
            let mut errors = ValidationErrors::new();

            errors.add(
                "favorite",
                ValidationError::new("not_found")
                    .with_message("Favoriteable does not exist.".into()),
            );

            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn toggle(
        ctx: &AppContext,
        params: ToggleFavoriteActionParameters,
    ) -> Result<Option<favorite::Model>, AppError> {
        Self::validate(&ctx.db, &params).await?;

        let favoriteable_id = match &params.favoriteable {
            FavoriteableType::Entry(id) => *id,
        };

        let txn = ctx.db.begin().await?;

        let existing = favorite::Entity::find()
            .filter(favorite::Column::UserId.eq(params.user_id))
            .filter(
                favorite::Column::FavoriteableType
                    .eq::<favorite::FavoriteableType>(params.favoriteable.into()),
            )
            .filter(favorite::Column::FavoriteableId.eq(favoriteable_id))
            .one(&txn)
            .await?;

        if let Some(model) = existing {
            model.clone().delete(&txn).await?;
            model.reorder_after_delete(&txn).await?;
            txn.commit().await?;
            return Ok(None);
        }

        let max_position: i32 = favorite::Entity::find()
            .select_only()
            .column(favorite::Column::Position)
            .filter(favorite::Column::UserId.eq(params.user_id))
            .filter(
                favorite::Column::FavoriteableType
                    .eq::<favorite::FavoriteableType>(params.favoriteable.into()),
            )
            .order_by_desc(favorite::Column::Position)
            .into_tuple()
            .one(&txn)
            .await?
            .unwrap_or(0)
            + 1;

        let model = favorite::ActiveModel {
            favoriteable_type: Set(params.favoriteable.into()),
            favoriteable_id: Set(favoriteable_id),
            user_id: Set(params.user_id),
            position: Set(max_position),
            ..Default::default()
        };

        let model = model.insert(&txn).await?;

        txn.commit().await?;

        Ok(Some(model))
    }

    pub async fn update_favorite(
        ctx: &AppContext,
        mut favorite: favorite::Model,
        params: UpdateFavoriteActionParameters,
    ) -> Result<favorite::Model, AppError> {
        if let Err(errors) = params.validate() {
            return Err(AppError::Validation(errors));
        }

        let txn = ctx.db.begin().await?;

        if let Some(position) = params.position {
            favorite = favorite.move_to(&txn, position).await?;
        }

        txn.commit().await?;

        Ok(favorite)
    }
}
