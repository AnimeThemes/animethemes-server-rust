use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::AppError;
use crate::actions::entities::admin::featured_theme::{
    SaveFeaturedThemeAction, SaveFeaturedThemeInput,
};
use crate::entities::admin::featuredtheme;
use crate::graphql::types::admin::featuredtheme::FeaturedTheme;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::admin::featuredtheme::FeaturedThemePolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(Default)]
pub struct FeaturedThemeMutation;

#[Object]
impl FeaturedThemeMutation {
    async fn save_featured_theme(
        &self,
        ctx: &Context<'_>,
        id: Option<u64>,
        input: SaveFeaturedThemeInput,
    ) -> Result<FeaturedTheme> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let model = if let Some(id) = id {
            Some(
                featuredtheme::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::from(AppError::NotFound))?,
            )
        } else {
            None
        };

        if let Some(featuredtheme) = &model {
            FeaturedThemePolicy::check(Some(user), PolicyAction::Update, Some(featuredtheme))
                .authorize()
                .extend()?;
        } else {
            FeaturedThemePolicy::check(Some(user), PolicyAction::Create, None)
                .authorize()
                .extend()?;
        }

        let featuredtheme =
            SaveFeaturedThemeAction::save(ctx.data_unchecked::<AppContext>(), model, input, &user)
                .await
                .extend()?;

        Ok(featuredtheme.into())
    }

    async fn delete_featured_theme(&self, ctx: &Context<'_>, id: u64) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let featuredtheme = featuredtheme::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        FeaturedThemePolicy::check(Some(user), PolicyAction::Delete, Some(&featuredtheme))
            .authorize()
            .extend()?;

        SaveFeaturedThemeAction::delete(ctx.data_unchecked::<AppContext>(), featuredtheme, &user)
            .await
            .extend()?;

        Ok(true)
    }
}
