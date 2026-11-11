use loco_rs::app::AppContext;
use loco_rs::{Error, Result};
use serde::{Deserialize, Deserializer};

use crate::storage::StorageConfig;

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub storage: StorageConfig,
}

pub fn null_as_empty<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.unwrap_or_default())
}

pub fn load_settings(ctx: AppContext) -> Result<Settings> {
    let settings: Settings = serde_json::from_value(
        ctx.config
            .settings
            .clone()
            .ok_or_else(|| Error::Message("Configuration missing: settings not found".into()))?,
    )
    .map_err(|e| Error::Message(format!("Invalid configuration: {e}")))?;

    Ok(settings)
}
