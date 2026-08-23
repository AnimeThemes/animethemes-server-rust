use crate::entities::user::like;
use crate::graphql::types::user::like::Like;
use async_graphql::{Context, Error, Object, OneofObject, Result};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
};

use crate::middlewares::current_user::CurrentUser;
use crate::policies::AppError;

#[derive(OneofObject)]
enum LikeableType {
    Entry(u64),
}

#[derive(Default)]
pub struct LikeMutation;

#[Object]
impl LikeMutation {
    async fn toggle_like(&self, ctx: &Context<'_>, like: LikeableType) -> Result<Option<Like>> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let (likeable_type, likeable_id) = match like {
            LikeableType::Entry(id) => ("animethemeentry", id),
        };

        let existing = like::Entity::find()
            .filter(like::Column::UserId.eq(user.user.id))
            .filter(like::Column::LikeableType.eq(likeable_type))
            .filter(like::Column::LikeableId.eq(likeable_id))
            .one(db)
            .await?;

        if let Some(model) = existing {
            model.delete(db).await?;
            return Ok(None);
        }

        let model = like::ActiveModel {
            likeable_type: Set(likeable_type.to_string()),
            likeable_id: Set(likeable_id),
            user_id: Set(user.user.clone().id),
            ..Default::default()
        };

        let model = model.insert(db).await?;

        Ok(Some(model.into()))
    }
}
