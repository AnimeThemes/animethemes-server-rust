use crate::AppError;
use crate::entities::user::favorite;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
};

pub struct ToggleFavoriteActionParameters<'a> {
    pub favoriteable_type: &'a str,
    pub favoriteable_id: u64,
    pub user_id: u64,
}

pub struct FavoriteAction;

impl FavoriteAction {
    pub async fn toggle(
        db: &DatabaseConnection,
        params: ToggleFavoriteActionParameters<'_>,
    ) -> Result<Option<favorite::Model>, AppError> {
        let existing = favorite::Entity::find()
            .filter(favorite::Column::UserId.eq(params.user_id))
            .filter(favorite::Column::FavoriteableType.eq(params.favoriteable_type))
            .filter(favorite::Column::FavoriteableId.eq(params.favoriteable_id))
            .one(db)
            .await?;

        if let Some(model) = existing {
            model.delete(db).await?;
            return Ok(None);
        }

        let model = favorite::ActiveModel {
            favoriteable_type: Set(params.favoriteable_type.to_string()),
            favoriteable_id: Set(params.favoriteable_id),
            user_id: Set(params.user_id),
            ..Default::default()
        };

        let model = model.insert(db).await?;

        Ok(Some(model))
    }
}
