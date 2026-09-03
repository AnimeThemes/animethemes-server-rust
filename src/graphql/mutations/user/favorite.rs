use crate::actions::entities::user::favorite::{FavoriteAction, ToggleFavoriteActionParameters};
use crate::graphql::types::user::favorite::Favorite;
use async_graphql::{Context, Error, Object, OneofObject, Result, ResultExt};
use sea_orm::DatabaseConnection;

use crate::AppError;
use crate::middlewares::current_user::CurrentUser;

#[derive(OneofObject)]
enum FavoriteableType {
    Entry(u64),
}

#[derive(Default)]
pub struct FavoriteMutation;

#[Object]
impl FavoriteMutation {
    async fn toggle_favorite(
        &self,
        ctx: &Context<'_>,
        favorite: FavoriteableType,
    ) -> Result<Option<Favorite>> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?;

        let db = ctx.data::<DatabaseConnection>()?;

        let (favoriteable_type, favoriteable_id) = match favorite {
            FavoriteableType::Entry(id) => ("entry", id),
        };

        let model = FavoriteAction::toggle(
            db,
            ToggleFavoriteActionParameters {
                favoriteable_type,
                favoriteable_id,
                user_id: user.user.clone().id,
            },
        )
        .await
        .extend()?;

        Ok(model.map(Into::into))
    }
}
