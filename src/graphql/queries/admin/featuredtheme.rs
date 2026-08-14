use async_graphql::{Context, Object, Result};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::admin::featuredtheme, graphql::types::admin::featuredtheme::FeaturedTheme,
    scopes::admin::featuredtheme::current_featured_theme,
};

#[derive(Default)]
pub struct FeaturedThemeQuery;

#[Object]
impl FeaturedThemeQuery {
    async fn current_featured_theme(&self, ctx: &Context<'_>) -> Result<Option<FeaturedTheme>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let featured_theme = featuredtheme::Entity::find()
            .filter(current_featured_theme())
            .one(db)
            .await?;

        Ok(featured_theme.map(Into::into))
    }
}
