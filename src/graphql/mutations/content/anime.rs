use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;

use crate::AppError;
use crate::actions::entities::content::anime::{SaveAnimeAction, SaveAnimeInput};
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

        AnimePolicy::check(Some(user), PolicyAction::Create, None)
            .authorize()
            .extend()?;

        let anime = SaveAnimeAction::save(ctx.data_unchecked::<AppContext>(), id, input, &user)
            .await
            .extend()?;

        Ok(anime.into())
    }
}
