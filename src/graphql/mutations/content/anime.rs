use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::AppError;
use crate::actions::entities::content::anime::{SaveAnimeAction, SaveAnimeInput};
use crate::entities::content::anime;
use crate::graphql::types::content::anime::Anime;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::content::anime::AnimePolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(Default)]
pub struct AnimeMutation;

#[Object]
impl AnimeMutation {
    async fn save_anime(
        &self,
        ctx: &Context<'_>,
        id: Option<u64>,
        input: SaveAnimeInput,
    ) -> Result<Anime> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let model = if let Some(id) = id {
            Some(
                anime::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::from(AppError::NotFound))?,
            )
        } else {
            None
        };

        if let Some(anime) = &model {
            AnimePolicy::check(Some(user), PolicyAction::Update, Some(anime))
                .authorize()
                .extend()?;
        } else {
            AnimePolicy::check(Some(user), PolicyAction::Create, None)
                .authorize()
                .extend()?;
        }

        let anime = SaveAnimeAction::save(ctx.data_unchecked::<AppContext>(), model, input, &user)
            .await
            .extend()?;

        Ok(anime.into())
    }

    async fn delete_anime(
        &self,
        ctx: &Context<'_>,
        id: u64,
        #[graphql(default = false)] force_delete: bool,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let anime = anime::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        AnimePolicy::check(Some(user), PolicyAction::Delete, Some(&anime))
            .authorize()
            .extend()?;

        SaveAnimeAction::delete(
            ctx.data_unchecked::<AppContext>(),
            anime,
            force_delete,
            &user,
        )
        .await
        .extend()?;

        Ok(true)
    }
}
