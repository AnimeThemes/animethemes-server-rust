use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::AppError;
use crate::actions::entities::content::theme::{SaveThemeAction, SaveThemeInput};
use crate::entities::content::theme;
use crate::graphql::types::content::theme::Theme;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::content::theme::ThemePolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(Default)]
pub struct ThemeMutation;

#[Object]
impl ThemeMutation {
    async fn save_theme(
        &self,
        ctx: &Context<'_>,
        id: Option<u64>,
        input: SaveThemeInput,
    ) -> Result<Theme> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let model = if let Some(id) = id {
            Some(
                theme::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::from(AppError::NotFound))?,
            )
        } else {
            None
        };

        if let Some(theme) = &model {
            ThemePolicy::check(Some(user), PolicyAction::Update, Some(theme))
                .authorize()
                .extend()?;
        } else {
            ThemePolicy::check(Some(user), PolicyAction::Create, None)
                .authorize()
                .extend()?;
        }

        let theme = SaveThemeAction::save(ctx.data_unchecked::<AppContext>(), model, input, &user)
            .await
            .extend()?;

        Ok(theme.into())
    }

    async fn delete_theme(
        &self,
        ctx: &Context<'_>,
        id: u64,
        #[graphql(default = false)] force_delete: bool,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let theme = theme::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        ThemePolicy::check(Some(user), PolicyAction::Delete, Some(&theme))
            .authorize()
            .extend()?;

        SaveThemeAction::delete(
            ctx.data_unchecked::<AppContext>(),
            theme,
            force_delete,
            &user,
        )
        .await
        .extend()?;

        Ok(true)
    }
}
