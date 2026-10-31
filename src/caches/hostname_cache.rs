use std::{collections::HashMap, env, net::IpAddr, sync::Arc, time::Duration};

use arc_swap::ArcSwap;
use tokio::net::lookup_host;

#[derive(Clone)]
pub struct HostnameCache {
    ips: Arc<ArcSwap<HashMap<IpAddr, Arc<str>>>>,
}

impl HostnameCache {
    pub async fn new() -> Self {
        let hostnames: Vec<String> = env::var("HOSTNAMES_BYPASS")
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .collect();

        let cache = Self {
            ips: Arc::new(ArcSwap::from_pointee(HashMap::new())),
        };

        // Initial DNS resolution
        cache.refresh(&hostnames).await;

        if !hostnames.is_empty() {
            let background = cache.clone();

            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_mins(10)).await;
                    background.refresh(&hostnames).await;
                }
            });
        }

        cache
    }

    async fn refresh(&self, hostnames: &[String]) {
        let mut ips = HashMap::new();

        for hostname in hostnames {
            match lookup_host((hostname.as_str(), 0)).await {
                Ok(addresses) => {
                    let name: Arc<str> = Arc::from(hostname.as_str());

                    for address in addresses {
                        ips.entry(address.ip()).or_insert_with(|| Arc::clone(&name));
                    }
                }
                Err(err) => {
                    tracing::warn!("Failed to resolve {hostname}: {err}");
                }
            }
        }

        // Atomic cache replacement
        self.ips.store(Arc::new(ips));
    }

    pub fn is_allowed(&self, ip: IpAddr) -> bool {
        self.ips.load().contains_key(&ip)
    }
}
