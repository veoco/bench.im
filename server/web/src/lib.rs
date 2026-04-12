use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::str::FromStr;

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

use server_service::sea_orm::{ConnectOptions, Database};
use server_service::{Mutation as MutationCore, ApplicationService, init_searcher};

mod admin;
mod application;
mod assets;
mod config;
mod error;
mod extractors;
mod index;
mod machines;
mod pings;
mod state;
mod targets;
mod templates;

pub use config::Config;
pub use error::{ApiError, render_template};
pub use state::AppState;

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
        let _ = MutationCore::delete_expire_pings(state.db()).await;
        // 清理过期申请者（1天未更新）
        let _ = ApplicationService::clean_expired_applicants(state.db()).await;
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

    // 加载配置
    let config = Config::from_env();
    let addr = config.listen_address;

    info!("Listening on http://{}/", addr);

    // 初始化 IP 地理位置搜索器
    let enable_apply = match init_searcher(&config.ip2region_v4_path, &config.ip2region_v6_path) {
        Ok(_) => {
            if config.enable_apply {
                info!("Apply feature: enabled");
            } else {
                info!("Apply feature: disabled (set ENABLE_APPLY=true to enable)");
            }
            config.enable_apply
        }
        Err(e) => {
            tracing::warn!("Failed to init ip2region searcher: {}", e);
            tracing::warn!("Apply feature will be disabled");
            false
        }
    };

    // 数据库连接
    let opt = ConnectOptions::new(config.database_url.clone());
    let conn = Database::connect(opt)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    // 创建 AppState（enable_apply 可能因初始化失败而改变）
    let mut config = config;
    config.enable_apply = enable_apply;
    let state = Arc::new(AppState::new(conn, config));

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
