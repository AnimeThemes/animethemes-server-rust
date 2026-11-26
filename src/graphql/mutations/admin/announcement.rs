use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::AppError;
use crate::actions::entities::admin::announcement::{
    SaveAnnouncementAction, SaveAnnouncementInput,
};
use crate::entities::admin::announcement;
use crate::graphql::types::admin::announcement::Announcement;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::admin::announcement::AnnouncementPolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(Default)]
pub struct AnnouncementMutation;

#[Object]
impl AnnouncementMutation {
    async fn save_announcement(
        &self,
        ctx: &Context<'_>,
        id: Option<u64>,
        input: SaveAnnouncementInput,
    ) -> Result<Announcement> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let model = if let Some(id) = id {
            Some(
                announcement::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::from(AppError::NotFound))?,
            )
        } else {
            None
        };

        if let Some(announcement) = &model {
            AnnouncementPolicy::check(Some(user), PolicyAction::Update, Some(announcement))
                .authorize()
                .extend()?;
        } else {
            AnnouncementPolicy::check(Some(user), PolicyAction::Create, None)
                .authorize()
                .extend()?;
        }

        let announcement =
            SaveAnnouncementAction::save(ctx.data_unchecked::<AppContext>(), model, input, &user)
                .await
                .extend()?;

        Ok(announcement.into())
    }

    async fn delete_announcement(&self, ctx: &Context<'_>, id: u64) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let announcement = announcement::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        AnnouncementPolicy::check(Some(user), PolicyAction::Delete, Some(&announcement))
            .authorize()
            .extend()?;

        SaveAnnouncementAction::delete(ctx.data_unchecked::<AppContext>(), announcement, &user)
            .await
            .extend()?;

        Ok(true)
    }
}
