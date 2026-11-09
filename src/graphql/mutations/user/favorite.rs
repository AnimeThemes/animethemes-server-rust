use crate::actions::entities::user::favorite::{
    self as FavoriteActions, FavoriteAction, ToggleFavoriteActionParameters,
    UpdateFavoriteActionParameters,
};
use crate::entities::user::favorite;
use crate::graphql::types::user::favorite::Favorite;
use crate::policies::user::favorite::FavoritePolicy;
use crate::policies::{Policy, PolicyAction};
use async_graphql::{Context, Error, InputObject, Object, OneofObject, Result, ResultExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::AppError;
use crate::middlewares::current_user::CurrentUser;

#[derive(OneofObject)]
enum FavoriteableType {
    Entry(u64),
}

impl Into<FavoriteActions::FavoriteableType> for FavoriteableType {
    fn into(self) -> FavoriteActions::FavoriteableType {
        match self {
            FavoriteableType::Entry(id) => FavoriteActions::FavoriteableType::Entry(id),
        }
    }
}

#[derive(InputObject)]
struct UpdateFavoriteInput {
    position: Option<i32>,
}

#[derive(Default)]
pub struct FavoriteMutation;

#[Object]
impl FavoriteMutation {
    async fn toggle_favorite(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "favorite")] favoriteable: FavoriteableType,
    ) -> Result<Option<Favorite>> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?;

        let db = ctx.data::<DatabaseConnection>()?;

        FavoritePolicy::check(Some(user), PolicyAction::Create, None)
            .authorize()
            .extend()?;

        let model = FavoriteAction::toggle(
            db,
            ToggleFavoriteActionParameters {
                favoriteable: favoriteable.into(),
                user_id: user.user.clone().id,
            },
        )
        .await
        .extend()?;

        Ok(model.map(Into::into))
    }

    async fn update_favorite(
        &self,
        ctx: &Context<'_>,
        id: u64,
        input: UpdateFavoriteInput,
    ) -> Result<Favorite> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))
            .extend()?;

        let db = ctx.data::<DatabaseConnection>()?;

        let favorite = favorite::Entity::find()
            .filter(favorite::Column::Id.eq(id))
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        FavoritePolicy::check(Some(user), PolicyAction::Update, Some(&favorite))
            .authorize()
            .extend()?;

        let model = FavoriteAction::update_favorite(
            db,
            favorite,
            UpdateFavoriteActionParameters {
                position: input.position,
            },
        )
        .await
        .extend()?;

        Ok(model.into())
    }
}
