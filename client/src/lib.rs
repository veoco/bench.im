mod models;
mod ping_native;
mod ping_system;

use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;

use surge_ping::{Client, Config, ICMP};
use tokio::sync::Semaphore;

pub use models::{Message, PingData, Target};

use log::{debug, info};

pub struct BimClient {
    pub mid: i32,
    pub token: String,
    pub server_url: String,
    pub client: reqwest::Client,
    pub semaphore: Arc<Semaphore>,
}

impl BimClient {
    pub async fn new(mid: i32, token: String, server_url: String) -> Result<Self, String> {
        let client = reqwest::Client::new();
        let semaphore = Arc::new(Semaphore::new(1));

        let token = format!("{}:{}", mid, token);

        Ok(Self {
            mid,
            token,
            server_url,
            client,
            semaphore,
        })
    }

    pub async fn get_targets(&self) -> Result<Vec<Target>, String> {
        let url = format!("{}/api/client/targets/", self.server_url);
        let r = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|_| "Network error")?;

        debug!("Status code: {}", r.status());
        if r.status() != 200 {
            return Err("Invalid mid or token".to_string());
        }

        let targets = r
            .json::<Vec<Target>>()
            .await
            .map_err(|_| "Upgrade required")?;
        Ok(targets)
    }

    pub async fn post_target_data(&self, target_id: i32, data: PingData) {
        let permit = match self.semaphore.acquire().await {
            Ok(p) => p,
            _ => {
                debug!("CC Acquire semaphore failed");

                return;
            }
        };

        let url = format!("{}/api/client/targets/{}", self.server_url, target_id);

        let r = match self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .timeout(Duration::from_secs(10))
            .json(&data)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                debug!("Add target data failed: {e}");
                return;
            }
        };

        drop(permit);

        debug!("Status code: {}", r.status());
        match r.json::<Message>().await {
            Ok(_) => {
                debug!("Add target {target_id} data success");
            }
            Err(e) => {
                debug!("Add target data failed: {e}");
                info!("Upgrade required");
            }
        };
        return;
    }
}

#[derive(Clone)]
pub enum PingMode {
    Native {
        v4_client: Arc<Client>,
        v6_client: Arc<Client>,
    },
    System,
}

impl PingMode {
    pub async fn detect() -> Self {
        // 创建 IPv4 Client（默认配置）
        let v4_client = match Client::new(&Default::default()) {
            Ok(client) => Arc::new(client),
            Err(_) => return PingMode::System,
        };

        // 创建 IPv6 Client
        let v6_config = Config::builder().kind(ICMP::V6).build();
        let v6_client = match Client::new(&v6_config) {
            Ok(client) => Arc::new(client),
            Err(_) => return PingMode::System,
        };

        // 测试 IPv4 回环地址
        let ident = surge_ping::PingIdentifier(0);
        let mut v4_pinger = v4_client
            .pinger(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), ident)
            .await;

        if v4_pinger.ping(surge_ping::PingSequence(0), &[]).await.is_err() {
            return PingMode::System;
        }

        // 测试 IPv6 回环地址
        let ident = surge_ping::PingIdentifier(0);
        let mut v6_pinger = v6_client
            .pinger(IpAddr::V6(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), ident)
            .await;

        if v6_pinger.ping(surge_ping::PingSequence(0), &[]).await.is_err() {
            return PingMode::System;
        }

        // IPv4 和 IPv6 都测试通过
        PingMode::Native {
            v4_client,
            v6_client,
        }
    }

    pub async fn ping(
        &self,
        target_ip: IpAddr,
        ipv6: bool,
        semaphore: Arc<Semaphore>,
        target_id: i32,
        cc: Arc<BimClient>,
    ) -> Option<PingData> {
        match self {
            PingMode::Native {
                v4_client,
                v6_client,
            } => {
                ping_native::ping_native(
                    v4_client, v6_client, target_ip, ipv6, semaphore, target_id, cc,
                )
                .await
            }
            PingMode::System => {
                ping_system::ping_system(target_ip, ipv6, semaphore, target_id, cc).await
            }
        }
    }
}

/// 域名解析，分别获取 IPv4 和 IPv6 地址
pub async fn resolve_domain(domain: &str) -> Result<(Option<IpAddr>, Option<IpAddr>), String> {
    use tokio::net::lookup_host;
    
    let addrs: Vec<_> = lookup_host(format!("{}:0", domain))
        .await
        .map_err(|e| e.to_string())?
        .collect();
    
    let v4 = addrs.iter()
        .find_map(|sa| match sa.ip() {
            IpAddr::V4(v4) => Some(IpAddr::V4(v4)),
            _ => None,
        });
    
    let v6 = addrs.iter()
        .find_map(|sa| match sa.ip() {
            IpAddr::V6(v6) => Some(IpAddr::V6(v6)),
            _ => None,
        });
    
    Ok((v4, v6))
}
