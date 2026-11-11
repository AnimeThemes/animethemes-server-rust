use crate::AppError;
use crate::entities::content::entry;
use crate::entities::user::favorite;
use crate::rules::validation_error::ValidationError;
use crate::traits::sortable::Sortable;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
    QueryOrder, QuerySelect, SelectExt, TransactionTrait,
};

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

pub struct UpdateFavoriteActionParameters {
    pub position: Option<i32>,
}

pub struct FavoriteAction;

impl FavoriteAction {
    async fn validate(
        db: &DatabaseConnection,
        params: &ToggleFavoriteActionParameters,
    ) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let favoriteable_exists = match &params.favoriteable {
            FavoriteableType::Entry(id) => {
                entry::Entity::find()
                    .filter(entry::Column::Id.eq(*id))
                    .exists(db)
                    .await?
            }
        };

        if !favoriteable_exists {
            errors.push(ValidationError::new(
                "favorite",
                vec!["Favoriteable does not exist."],
            ));
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn toggle(
        db: &DatabaseConnection,
        params: ToggleFavoriteActionParameters,
    ) -> Result<Option<favorite::Model>, AppError> {
        Self::validate(db, &params).await?;

        let favoriteable_id = match &params.favoriteable {
            FavoriteableType::Entry(id) => *id,
        };

        let txn = db.begin().await?;

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

    async fn validate_update(params: &UpdateFavoriteActionParameters) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let mut position_errors = Vec::new();

        if let Some(position) = params.position {
            if position < 1 {
                position_errors.push("The position must be greater than 0.");
            }
        }

        if !position_errors.is_empty() {
            errors.push(ValidationError::new("position", position_errors));
        }

        if !errors.is_empty() {
            return Err(AppError::Validation(errors));
        }

        Ok(())
    }

    pub async fn update_favorite(
        db: &DatabaseConnection,
        mut favorite: favorite::Model,
        params: UpdateFavoriteActionParameters,
    ) -> Result<favorite::Model, AppError> {
        Self::validate_update(&params).await?;

        let txn = db.begin().await?;

        if let Some(position) = params.position {
            favorite = favorite.move_to(&txn, position).await?;
        }

        txn.commit().await?;

        Ok(favorite)
    }
}
