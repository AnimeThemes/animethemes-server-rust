use async_graphql::{Context, Error, Object, Result, ResultExt};
use loco_rs::app::AppContext;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::AppError;
use crate::actions::entities::content::series::{SaveSeriesAction, SaveSeriesInput};
use crate::entities::content::series;
use crate::graphql::types::content::series::Series;
use crate::middlewares::current_user::CurrentUser;
use crate::policies::content::series::SeriesPolicy;
use crate::policies::{Policy, PolicyAction};

#[derive(Default)]
pub struct SeriesMutation;

#[Object]
impl SeriesMutation {
    async fn save_series(
        &self,
        ctx: &Context<'_>,
        id: Option<u64>,
        input: SaveSeriesInput,
    ) -> Result<Series> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let model = if let Some(id) = id {
            Some(
                series::Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or_else(|| Error::from(AppError::NotFound))?,
            )
        } else {
            None
        };

        if let Some(series) = &model {
            SeriesPolicy::check(Some(user), PolicyAction::Update, Some(series))
                .authorize()
                .extend()?;
        } else {
            SeriesPolicy::check(Some(user), PolicyAction::Create, None)
                .authorize()
                .extend()?;
        }

        let series =
            SaveSeriesAction::save(ctx.data_unchecked::<AppContext>(), model, input, &user)
                .await
                .extend()?;

        Ok(series.into())
    }

    async fn delete_series(
        &self,
        ctx: &Context<'_>,
        id: u64,
        #[graphql(default = false)] force_delete: bool,
    ) -> Result<bool> {
        let user = ctx
            .data::<CurrentUser>()
            .map_err(|_| Error::from(AppError::Unauthenticated))?;

        let db = ctx.data::<DatabaseConnection>()?;

        let series = series::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::from(AppError::NotFound))?;

        SeriesPolicy::check(Some(user), PolicyAction::Delete, Some(&series))
            .authorize()
            .extend()?;

        SaveSeriesAction::delete(
            ctx.data_unchecked::<AppContext>(),
            series,
            force_delete,
            &user,
        )
        .await
        .extend()?;

        Ok(true)
    }
}
