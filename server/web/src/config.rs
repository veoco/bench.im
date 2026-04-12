use std::net::SocketAddr;

use crate::IpRange;

/// 应用配置
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub listen_address: SocketAddr,
    pub admin_password: String,
    pub site_name: String,
    pub server_url: String,
    pub enable_apply: bool,
    pub ip2region_v4_path: String,
    pub ip2region_v6_path: String,
    pub trusted_proxies: Option<Vec<IpRange>>,
}

impl Config {
    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let admin_password =
            std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "fake-admin-password".to_string());

        // 安全警告
        if admin_password == "fake-admin-password" {
            tracing::warn!("Using default admin password! Please change it in production.");
        }

        let trusted_proxies = std::env::var("TRUSTED_PROXIES")
            .ok()
            .and_then(|v| parse_trusted_proxies(&v));

        Self {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL is not set in .env file"),
            listen_address: std::env::var("LISTEN_ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1:3000".to_string())
                .parse()
                .expect("Invalid LISTEN_ADDRESS"),
            admin_password,
            site_name: std::env::var("SITE_NAME").unwrap_or_else(|_| "Bench.im".to_string()),
            server_url: std::env::var("SERVER_URL")
                .unwrap_or_else(|_| "https://your-server.fake-url".to_string()),
            enable_apply: std::env::var("ENABLE_APPLY")
                .map(|v| v == "true")
                .unwrap_or(false),
            ip2region_v4_path: std::env::var("IP2REGION_V4_DB")
                .unwrap_or_else(|_| "server/ip2region_v4.xdb".to_string()),
            ip2region_v6_path: std::env::var("IP2REGION_V6_DB")
                .unwrap_or_else(|_| "server/ip2region_v6.xdb".to_string()),
            trusted_proxies,
        }
    }
}

/// 解析可信代理配置
fn parse_trusted_proxies(value: &str) -> Option<Vec<IpRange>> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }

    let mut ranges = Vec::new();
    for part in value.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        match part.parse::<IpRange>() {
            Ok(range) => ranges.push(range),
            Err(e) => {
                tracing::warn!("Failed to parse trusted proxy '{}': {}", part, e);
            }
        }
    }

    if ranges.is_empty() {
        None
    } else {
        tracing::info!("Trusted proxies configured: {}", value);
        Some(ranges)
    }
}
