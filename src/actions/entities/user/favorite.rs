use crate::AppError;
use crate::entities::content::entry;
use crate::entities::user::favorite;
use crate::rules::validation_error::ValidationError;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
    SelectExt,
};

pub enum FavoriteableType {
    Entry(u64),
}

impl FavoriteableType {
    pub fn to_morph(&self) -> &str {
        match self {
            FavoriteableType::Entry(_) => "entry",
        }
    }
}

pub struct ToggleFavoriteActionParameters {
    pub favoriteable: FavoriteableType,
    pub user_id: u64,
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

        let existing = favorite::Entity::find()
            .filter(favorite::Column::UserId.eq(params.user_id))
            .filter(favorite::Column::FavoriteableType.eq(params.favoriteable.to_morph()))
            .filter(favorite::Column::FavoriteableId.eq(favoriteable_id))
            .one(db)
            .await?;

        if let Some(model) = existing {
            model.delete(db).await?;
            return Ok(None);
        }

        let model = favorite::ActiveModel {
            favoriteable_type: Set(params.favoriteable.to_morph().to_string()),
            favoriteable_id: Set(favoriteable_id),
            user_id: Set(params.user_id),
            ..Default::default()
        };

        let model = model.insert(db).await?;

        Ok(Some(model))
    }
}
