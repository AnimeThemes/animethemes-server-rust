use std::collections::BTreeMap;

use loco_rs::app::AppContext;
use loco_rs::storage::drivers::{self, StoreDriver};
use loco_rs::storage::strategies::single::SingleStrategy;
use loco_rs::storage::{self, Storage};
use loco_rs::{Error, Result};
use serde::Deserialize;

use crate::settings::{Settings, null_as_empty};

#[derive(Clone, Debug, Deserialize)]
pub struct StorageConfig {
    pub default: String,
    pub disks: BTreeMap<String, DiskConfig>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "driver", rename_all = "lowercase")]
#[allow(dead_code)]
pub enum DiskConfig {
    // https://loco.rs/docs/how-to/configure-storage/#2-pick-a-driver
    Local {
        root: String,
        #[serde(default)]
        url: Option<String>,
        #[serde(default)]
        visibility: Option<String>,
    },
    S3 {
        #[serde(default, deserialize_with = "null_as_empty")]
        key: String,
        #[serde(default, deserialize_with = "null_as_empty")]
        secret: String,
        #[serde(default, deserialize_with = "null_as_empty")]
        region: String,
        #[serde(default, deserialize_with = "null_as_empty")]
        bucket: String,
        #[serde(default)]
        endpoint: Option<String>,
        #[serde(default)]
        url: Option<String>,
        #[serde(default)]
        visibility: Option<String>,
    },
}

pub fn create_storage(ctx: AppContext) -> Result<Storage> {
    let settings = ctx
        .shared_store
        .get::<Settings>()
        .ok_or_else(|| Error::Message("Settings not found".into()))?;

    let StorageConfig { default, disks } = settings.storage;

    if !disks.contains_key(&default) {
        return Err(Error::Message(
            "Default storage disk '{default}' not found".into(),
        ));
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
                if key.trim().is_empty()
                    || secret.trim().is_empty()
                    || region.trim().is_empty()
                    || bucket.trim().is_empty()
                {
                    tracing::warn!("Skipping storage disk '{name}': missing configuration");
                    continue;
                }

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
