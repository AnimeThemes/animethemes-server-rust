use std::path::Path;

use loco_rs::{Error, Result, app::AppContext, environment::Environment};
use sea_orm::{EntityTrait, IntoActiveModel};

use crate::entities::auth::role;

pub async fn seed_roles(ctx: &AppContext, base: &Path) -> Result<()> {
    if ctx.environment != Environment::Development {
        return Ok(());
    }

    let contents = tokio::fs::read(base.join("roles.yaml")).await?;

    let roles: Vec<role::Model> =
        serde_yaml::from_slice(&contents).map_err(|e| Error::Message(e.to_string()))?;

    let roles = roles
        .into_iter()
        .map(IntoActiveModel::into_active_model)
        .collect::<Vec<_>>();

    role::Entity::insert_many(roles).exec(&ctx.db).await?;

    Ok(())
}
