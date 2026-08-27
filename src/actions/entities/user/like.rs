use crate::AppError;
use crate::entities::user::like;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
};

pub struct ToggleLikeActionParameters<'a> {
    pub likeable_type: &'a str,
    pub likeable_id: u64,
    pub user_id: u64,
}

pub struct LikeAction;

impl LikeAction {
    pub async fn toggle(
        db: &DatabaseConnection,
        params: ToggleLikeActionParameters<'_>,
    ) -> Result<Option<like::Model>, AppError> {
        let existing = like::Entity::find()
            .filter(like::Column::UserId.eq(params.user_id))
            .filter(like::Column::LikeableType.eq(params.likeable_type))
            .filter(like::Column::LikeableId.eq(params.likeable_id))
            .one(db)
            .await?;

        if let Some(model) = existing {
            model.delete(db).await?;
            return Ok(None);
        }

        let model = like::ActiveModel {
            likeable_type: Set(params.likeable_type.to_string()),
            likeable_id: Set(params.likeable_id),
            user_id: Set(params.user_id),
            ..Default::default()
        };

        let model = model.insert(db).await?;

        Ok(Some(model))
    }
}
