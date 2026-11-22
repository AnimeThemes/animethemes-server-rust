use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;

use crate::AppError;
use crate::actions::entities::content::create_anime::{
    CreateAnimeAction, CreateAnimeActionParameters,
};
use crate::graphql::types::content::anime::Anime;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::content::anime::AnimePolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(Default)]
pub struct AnimeMutation;

#[Object]
impl AnimeMutation {
    async fn create_anime(
        &self,
        ctx: &Context<'_>,
        input: CreateAnimeActionParameters,
    ) -> Result<Anime> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        AnimePolicy::check(Some(user), PolicyAction::Create, None)
            .authorize()
            .extend()?;

        let anime = CreateAnimeAction::create(ctx.data_unchecked::<AppContext>(), input, &user)
            .await
            .extend()?;

        Ok(anime.into())
    }
}
