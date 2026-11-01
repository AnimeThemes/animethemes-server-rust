use std::collections::BTreeMap;

use loco_rs::{
    app::AppContext,
    storage::{
        self, Storage,
        drivers::{self, StoreDriver},
        strategies::single::SingleStrategy,
    },
};
use serde::Deserialize;

use crate::AppError;

#[derive(Debug, Deserialize)]
struct Settings {
    pub storage: StorageConfig,
}

#[derive(Debug, Deserialize)]
struct StorageConfig {
    pub default: String,
    pub disks: BTreeMap<String, DiskConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "driver", rename_all = "lowercase")]
#[allow(dead_code)]
enum DiskConfig {
    Local {
        root: String,
        #[serde(default)]
        url: Option<String>,
        #[serde(default)]
        visibility: Option<String>,
    },
    S3 {
        key: String,
        secret: String,
        region: String,
        bucket: String,
        #[serde(default)]
        endpoint: Option<String>,
        #[serde(default)]
        url: Option<String>,
        #[serde(default)]
        visibility: Option<String>,
    },
}

pub fn create_storage(ctx: AppContext) -> Result<Storage, AppError> {
    let settings: Settings = serde_json::from_value(ctx.config.settings.ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!(
            "Storage settings not found in configuration"
        ))
    })?)
    .map_err(|e| AppError::Internal(e.into()))?;

    let StorageConfig { default, disks } = settings.storage;

    if !disks.contains_key(&default) {
        return Err(AppError::Internal(anyhow::anyhow!(
            "Default storage disk '{default}' not found"
        )));
    }

    let mut stores: BTreeMap<String, Box<dyn StoreDriver>> = BTreeMap::new();

    for (name, disk) in disks {
        let store: Box<dyn StoreDriver> = match disk {
            DiskConfig::Local { root, .. } => drivers::local::new_with_prefix(root)?,
            DiskConfig::S3 {
                key,
                secret,
                region,
                bucket,
                endpoint,
                ..
            } => {
                let credentials = drivers::aws::Credential {
                    key_id: key,
                    secret_key: secret,
                    token: None,
                };

                match endpoint {
                    Some(endpoint) if !endpoint.is_empty() => {
                        drivers::aws::with_credentials_and_endpoint(
                            &bucket,
                            &region,
                            &endpoint,
                            credentials,
                        )?
                    }
                    _ => drivers::aws::with_credentials(&bucket, &region, credentials)?,
                }
            }
        };

        stores.insert(name, store);
    }

    Ok(storage::Storage::new(
        stores,
        Box::new(SingleStrategy::new(&default)),
    ))
}
