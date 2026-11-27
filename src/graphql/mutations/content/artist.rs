use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::AppError;
use crate::actions::entities::content::artist::{SaveArtistAction, SaveArtistInput};
use crate::entities::content::artist;
use crate::graphql::types::content::artist::Artist;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::content::artist::ArtistPolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(Default)]
pub struct ArtistMutation;

#[Object]
impl ArtistMutation {
    async fn save_artist(
        &self,
        ctx: &Context<'_>,
        id: Option<u64>,
        input: SaveArtistInput,
    ) -> Result<Artist> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let model = if let Some(id) = id {
            Some(
                artist::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::from(AppError::NotFound))?,
            )
        } else {
            None
        };

        if let Some(artist) = &model {
            ArtistPolicy::check(Some(user), PolicyAction::Update, Some(artist))
                .authorize()
                .extend()?;
        } else {
            ArtistPolicy::check(Some(user), PolicyAction::Create, None)
                .authorize()
                .extend()?;
        }

        let artist =
            SaveArtistAction::save(ctx.data_unchecked::<AppContext>(), model, input, &user)
                .await
                .extend()?;

        Ok(artist.into())
    }

    async fn delete_artist(
        &self,
        ctx: &Context<'_>,
        id: u64,
        #[graphql(default = false)] force_delete: bool,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let artist = artist::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        ArtistPolicy::check(Some(user), PolicyAction::Delete, Some(&artist))
            .authorize()
            .extend()?;

        SaveArtistAction::delete(
            ctx.data_unchecked::<AppContext>(),
            artist,
            force_delete,
            &user,
        )
        .await
        .extend()?;

        Ok(true)
    }
}
