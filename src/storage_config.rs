use std::collections::BTreeMap;

use loco_rs::{
    Error, Result,
    app::AppContext,
    storage::{
        self, Storage,
        drivers::{self, StoreDriver},
        strategies::single::SingleStrategy,
    },
};
use serde::{Deserialize, Deserializer};

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

fn null_as_empty<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.unwrap_or_default())
}

pub fn create_storage(ctx: AppContext) -> Result<Storage> {
    let settings: Settings =
        serde_json::from_value(ctx.config.settings.clone().ok_or_else(|| {
            Error::Message("Storage configuration missing: settings not found".into())
        })?)
        .map_err(|e| Error::Message(format!("Invalid storage configuration: {e}")))?;

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
