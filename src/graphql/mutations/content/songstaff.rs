use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::AppError;
use crate::actions::entities::content::songstaff::{SaveSongStaffAction, SaveSongStaffInput};
use crate::entities::content::song_staff;
use crate::graphql::types::content::song_staff::SongStaff;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::content::songstaff::SongStaffPolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(Default)]
pub struct SongStaffMutation;

#[Object]
impl SongStaffMutation {
    async fn save_song_staff(
        &self,
        ctx: &Context<'_>,
        id: Option<u64>,
        input: SaveSongStaffInput,
    ) -> Result<SongStaff> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let model = if let Some(id) = id {
            Some(
                song_staff::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::from(AppError::NotFound))?,
            )
        } else {
            None
        };

        if let Some(staff) = &model {
            SongStaffPolicy::check(Some(user), PolicyAction::Update, Some(staff))
                .authorize()
                .extend()?;
        } else {
            SongStaffPolicy::check(Some(user), PolicyAction::Create, None)
                .authorize()
                .extend()?;
        }

        let staff =
            SaveSongStaffAction::save(ctx.data_unchecked::<AppContext>(), model, input, &user)
                .await
                .extend()?;

        Ok(staff.into())
    }

    async fn delete_song_staff(
        &self,
        ctx: &Context<'_>,
        id: u64,
        #[graphql(default = false)] force_delete: bool,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let staff = song_staff::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        SongStaffPolicy::check(Some(user), PolicyAction::Delete, Some(&staff))
            .authorize()
            .extend()?;

        SaveSongStaffAction::delete(
            ctx.data_unchecked::<AppContext>(),
            staff,
            force_delete,
            &user,
        )
        .await
        .extend()?;

        Ok(true)
    }
}
