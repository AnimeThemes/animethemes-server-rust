use async_graphql::{Context, Object, Result};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::admin::featuredtheme,
    graphql::types::admin::currentfeaturedtheme::CurrentFeaturedTheme,
    scopes::admin::featuredtheme::current_featured_theme,
};

#[derive(Default)]
pub struct FeaturedThemeQuery;

#[Object]
impl FeaturedThemeQuery {
    async fn current_featured_theme(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<CurrentFeaturedTheme>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let model = featuredtheme::Entity::find()
            .filter(current_featured_theme())
            .one(db)
            .await?;

        Ok(model.map(Into::into))
    }
}
