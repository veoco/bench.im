use std::sync::Arc;
use std::{env, net::SocketAddr};

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
use server_service::Mutation as MutationCore;

mod admin;
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
        let _ = MutationCore::delete_expire_pings(&state.conn).await;
    }
}

fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        // 页面路由
        .merge(index::create_router())      // 首页 + 机器详情
        .merge(admin::create_router())      // 管理后台
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

    info!("Listening on http://{addr}/");

    let opt = ConnectOptions::new(db_url.clone());
    let conn = Database::connect(opt)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let state = Arc::new(AppState {
        conn,
        admin_password,
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
