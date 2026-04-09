use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::{env, str::FromStr};

use axum::{
    routing::get,
    Router,
};
use migration::{Migrator, MigratorTrait};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::time::interval;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use server_service::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use server_service::{Mutation as MutationCore, ApplicationService, init_searcher};

mod admin;
mod application;
mod assets;
mod extractors;
mod index;
mod machines;
mod pings;
mod targets;
mod templates;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub admin_password: String,
    pub site_name: String,
    pub enable_apply: bool,
    pub server_url: String,
    pub trusted_proxies: Option<Vec<IpRange>>,
}

/// IP 范围配置（支持单个 IP 或 CIDR）
#[derive(Clone)]
pub enum IpRange {
    Single(IpAddr),
    Cidr(ipnet::IpNet),
}

impl IpRange {
    /// 检查 IP 是否在此范围内
    pub fn contains(&self, ip: &IpAddr) -> bool {
        match self {
            IpRange::Single(addr) => addr == ip,
            IpRange::Cidr(net) => net.contains(ip),
        }
    }
}

impl FromStr for IpRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 尝试解析为 CIDR（如 10.0.0.0/8）
        if s.contains('/') {
            match s.parse::<ipnet::IpNet>() {
                Ok(net) => return Ok(IpRange::Cidr(net)),
                Err(e) => return Err(format!("Invalid CIDR {}: {}", s, e)),
            }
        }

        // 尝试解析为单个 IP
        match s.parse::<IpAddr>() {
            Ok(addr) => Ok(IpRange::Single(addr)),
            Err(e) => Err(format!("Invalid IP {}: {}", s, e)),
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
        info!("Trusted proxies configured: {}", value);
        Some(ranges)
    }
}

/// 检查 IP 是否在可信代理列表中
pub fn is_trusted_proxy(ip: &IpAddr, trusted: &[IpRange]) -> bool {
    trusted.iter().any(|range| range.contains(ip))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}

async fn clean_database(state: Arc<AppState>) {
    let mut it = interval(std::time::Duration::from_secs(300));

    loop {
        it.tick().await;
        // 清理过期 ping 数据
        let _ = MutationCore::delete_expire_pings(&state.conn).await;
        // 清理过期申请者（1天未更新）
        let _ = ApplicationService::clean_expired_applicants(&state.conn).await;
    }
}

fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        // 页面路由
        .merge(index::create_router())      // 首页 + 机器详情
        .merge(admin::create_router())      // 管理后台
        .merge(application::create_router()) // 申请加入
        // API 路由
        .merge(machines::create_router())   // machines API + 管理页面
        .merge(targets::create_router())    // targets API + 管理页面
        .merge(pings::create_router())      // pings API
        // 静态资源
        .route("/static/{*path}", get(assets::serve_static))
        .with_state(state)
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let addr = env::var("LISTEN_ADDRESS").unwrap_or(String::from("127.0.0.1:3000"));
    let admin_password = env::var("ADMIN_PASSWORD").unwrap_or(String::from("fake-admin-password"));
    let site_name = env::var("SITE_NAME").unwrap_or(String::from("Bench.im"));
    let server_url = env::var("SERVER_URL").unwrap_or(String::from("https://your-server.fake-url"));
    let v4_db_path = env::var("IP2REGION_V4_DB").unwrap_or_else(|_| "server/ip2region_v4.xdb".to_string());
    let v6_db_path = env::var("IP2REGION_V6_DB").unwrap_or_else(|_| "server/ip2region_v6.xdb".to_string());

    // 解析可信代理配置
    let trusted_proxies = env::var("TRUSTED_PROXIES")
        .ok()
        .and_then(|v| parse_trusted_proxies(&v));

    info!("Listening on http://{addr}/");

    // 初始化 IP 地理位置搜索器
    let enable_apply = match init_searcher(&v4_db_path, &v6_db_path) {
        Ok(_) => {
            let enabled = env::var("ENABLE_APPLY").unwrap_or_else(|_| "false".to_string()) == "true";
            if enabled {
                info!("Apply feature: enabled");
            } else {
                info!("Apply feature: disabled (set ENABLE_APPLY=true to enable)");
            }
            enabled
        }
        Err(e) => {
            tracing::warn!("Failed to init ip2region searcher: {}", e);
            tracing::warn!("Apply feature will be disabled");
            false
        }
    };

    let opt = ConnectOptions::new(db_url.clone());
    let conn = Database::connect(opt)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let state = Arc::new(AppState {
        conn,
        admin_password,
        site_name,
        enable_apply,
        server_url,
        trusted_proxies,
    });

    tokio::spawn(clean_database(state.clone()));

    let app = build_app(state);

    let listener = TcpListener::bind(addr).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
