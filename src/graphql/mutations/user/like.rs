use crate::actions::entities::user::like::{LikeAction, ToggleLikeActionParameters};
use crate::graphql::types::user::like::Like;
use async_graphql::{Context, Error, Object, OneofObject, Result};
use sea_orm::DatabaseConnection;

use crate::AppError;
use crate::middlewares::current_user::CurrentUser;

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

        let model = LikeAction::toggle(
            db,
            ToggleLikeActionParameters {
                likeable_type,
                likeable_id,
                user_id: user.user.clone().id,
            },
        )
        .await?;

        Ok(model.map(Into::into))
    }
}
