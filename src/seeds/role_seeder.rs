use std::path::Path;

use loco_rs::app::AppContext;
use loco_rs::{Error, Result};
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};

use crate::entities::auth::role;

pub async fn seed_roles(ctx: &AppContext, base: &Path) -> Result<()> {
    let contents = tokio::fs::read(base.join("roles.yaml")).await?;

    let roles: Vec<role::Model> =
        serde_yaml::from_slice(&contents).map_err(|e| Error::Message(e.to_string()))?;

    for model in roles {
        let existing = role::Entity::find()
            .filter(role::Column::Name.eq(model.name.clone()))
            .one(&ctx.db)
            .await?;

        let mut active = model.into_active_model();

        active.created_at = ActiveValue::NotSet;
        active.updated_at = ActiveValue::NotSet;

        match existing {
            Some(existing) => {
                active.id = ActiveValue::Set(existing.id);

                role::Entity::update(active).exec(&ctx.db).await?;
            }
            None => {
                active.id = ActiveValue::NotSet;

                role::Entity::insert(active).exec(&ctx.db).await?;
            }
        }
    }

    Ok(())
}
