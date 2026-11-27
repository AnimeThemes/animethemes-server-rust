use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::AppError;
use crate::actions::entities::content::entry::{SaveEntryAction, SaveEntryInput};
use crate::entities::content::entry;
use crate::graphql::types::content::entry::Entry;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::content::entry::EntryPolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(Default)]
pub struct EntryMutation;

#[Object]
impl EntryMutation {
    async fn save_entry(
        &self,
        ctx: &Context<'_>,
        id: Option<u64>,
        input: SaveEntryInput,
    ) -> Result<Entry> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let model = if let Some(id) = id {
            Some(
                entry::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::from(AppError::NotFound))?,
            )
        } else {
            None
        };

        if let Some(entry) = &model {
            EntryPolicy::check(Some(user), PolicyAction::Update, Some(entry))
                .authorize()
                .extend()?;
        } else {
            EntryPolicy::check(Some(user), PolicyAction::Create, None)
                .authorize()
                .extend()?;
        }

        let entry = SaveEntryAction::save(ctx.data_unchecked::<AppContext>(), model, input, &user)
            .await
            .extend()?;

        Ok(entry.into())
    }

    async fn delete_entry(
        &self,
        ctx: &Context<'_>,
        id: u64,
        #[graphql(default = false)] force_delete: bool,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let entry = entry::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        EntryPolicy::check(Some(user), PolicyAction::Delete, Some(&entry))
            .authorize()
            .extend()?;

        SaveEntryAction::delete(
            ctx.data_unchecked::<AppContext>(),
            entry,
            force_delete,
            &user,
        )
        .await
        .extend()?;

        Ok(true)
    }
}
