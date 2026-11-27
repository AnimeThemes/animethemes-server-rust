use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::AppError;
use crate::actions::entities::content::song::{SaveSongAction, SaveSongInput};
use crate::entities::content::song;
use crate::graphql::types::content::song::Song;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::content::song::SongPolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(Default)]
pub struct SongMutation;

#[Object]
impl SongMutation {
    async fn save_song(
        &self,
        ctx: &Context<'_>,
        id: Option<u64>,
        input: SaveSongInput,
    ) -> Result<Song> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let model = if let Some(id) = id {
            Some(
                song::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::from(AppError::NotFound))?,
            )
        } else {
            None
        };

        if let Some(song) = &model {
            SongPolicy::check(Some(user), PolicyAction::Update, Some(song))
                .authorize()
                .extend()?;
        } else {
            SongPolicy::check(Some(user), PolicyAction::Create, None)
                .authorize()
                .extend()?;
        }

        let song = SaveSongAction::save(ctx.data_unchecked::<AppContext>(), model, input, &user)
            .await
            .extend()?;

        Ok(song.into())
    }

    async fn delete_song(
        &self,
        ctx: &Context<'_>,
        id: u64,
        #[graphql(default = false)] force_delete: bool,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let song = song::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        SongPolicy::check(Some(user), PolicyAction::Delete, Some(&song))
            .authorize()
            .extend()?;

        SaveSongAction::delete(
            ctx.data_unchecked::<AppContext>(),
            song,
            force_delete,
            &user,
        )
        .await
        .extend()?;

        Ok(true)
    }
}
