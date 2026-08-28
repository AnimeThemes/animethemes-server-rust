use std::path::Path;

use loco_rs::{Error, Result, app::AppContext, environment::Environment};
use sea_orm::{ActiveValue::NotSet, EntityTrait, IntoActiveModel};

use crate::entities::auth::sanction;

pub async fn seed_sanctions(ctx: &AppContext, base: &Path) -> Result<()> {
    if ctx.environment != Environment::Development {
        return Ok(());
    }

    let contents = tokio::fs::read(base.join("sanctions.yaml")).await?;

    let sanctions: Vec<sanction::Model> =
        serde_yaml::from_slice(&contents).map_err(|e| Error::Message(e.to_string()))?;

    let sanctions = sanctions
        .into_iter()
        .map(|model| {
            let mut active = model.into_active_model();

            active.created_at = NotSet;
            active.updated_at = NotSet;

            active
        })
        .collect::<Vec<_>>();

    sanction::Entity::insert_many(sanctions)
        .exec(&ctx.db)
        .await?;

    Ok(())
}
