use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::AppError;
use crate::actions::entities::content::studio::{SaveStudioAction, SaveStudioInput};
use crate::entities::content::studio;
use crate::graphql::types::content::studio::Studio;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::content::studio::StudioPolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(Default)]
pub struct StudioMutation;

#[Object]
impl StudioMutation {
    async fn save_studio(
        &self,
        ctx: &Context<'_>,
        id: Option<u64>,
        input: SaveStudioInput,
    ) -> Result<Studio> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let model = if let Some(id) = id {
            Some(
                studio::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::from(AppError::NotFound))?,
            )
        } else {
            None
        };

        if let Some(studio) = &model {
            StudioPolicy::check(Some(user), PolicyAction::Update, Some(studio))
                .authorize()
                .extend()?;
        } else {
            StudioPolicy::check(Some(user), PolicyAction::Create, None)
                .authorize()
                .extend()?;
        }

        let studio =
            SaveStudioAction::save(ctx.data_unchecked::<AppContext>(), model, input, &user)
                .await
                .extend()?;

        Ok(studio.into())
    }

    async fn delete_studio(
        &self,
        ctx: &Context<'_>,
        id: u64,
        #[graphql(default = false)] force_delete: bool,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let studio = studio::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        StudioPolicy::check(Some(user), PolicyAction::Delete, Some(&studio))
            .authorize()
            .extend()?;

        SaveStudioAction::delete(
            ctx.data_unchecked::<AppContext>(),
            studio,
            force_delete,
            &user,
        )
        .await
        .extend()?;

        Ok(true)
    }
}
