use std::path::Path;

use loco_rs::{Error, Result, app::AppContext};
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};

use crate::entities::auth::sanction;

pub async fn seed_sanctions(ctx: &AppContext, base: &Path) -> Result<()> {
    let contents = tokio::fs::read(base.join("sanctions.yaml")).await?;

    let sanctions: Vec<sanction::Model> =
        serde_yaml::from_slice(&contents).map_err(|e| Error::Message(e.to_string()))?;

    for model in sanctions {
        let existing = sanction::Entity::find()
            .filter(sanction::Column::Name.eq(model.name.clone()))
            .one(&ctx.db)
            .await?;

        let mut active = model.into_active_model();

        active.created_at = ActiveValue::NotSet;
        active.updated_at = ActiveValue::NotSet;

        match existing {
            Some(existing) => {
                active.id = ActiveValue::Set(existing.id);

                sanction::Entity::update(active).exec(&ctx.db).await?;
            }
            None => {
                active.id = ActiveValue::NotSet;

                sanction::Entity::insert(active).exec(&ctx.db).await?;
            }
        }
    }

    Ok(())
}
