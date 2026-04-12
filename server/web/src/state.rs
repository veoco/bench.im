use std::sync::Arc;

use server_service::sea_orm::DatabaseConnection;
use server_service::service::{MachineService, TargetService, PingService, ApplicationService};
use server_service::output::MachineListItem;
use server_service::IpGeoService;

use crate::config::Config;
use crate::IpRange;

/// 应用状态 - 封装配置、数据库连接和外部服务
#[derive(Clone)]
pub struct AppState {
    /// 数据库连接 - 私有，强制通过 service 层访问
    db: DatabaseConnection,
    /// 配置 - 使用 Arc 共享
    config: Arc<Config>,
    /// IP 地理位置服务
    ip_geo: Arc<IpGeoService>,
}

impl AppState {
    /// 创建新的 AppState
    pub fn new(db: DatabaseConnection, config: Config, ip_geo: Arc<IpGeoService>) -> Self {
        Self {
            db,
            config: Arc::new(config),
            ip_geo,
        }
    }

    // === 配置访问方法 ===

    /// 获取站点名称
    pub fn site_name(&self) -> &str {
        &self.config.site_name
    }

    /// 获取服务器 URL
    pub fn server_url(&self) -> &str {
        &self.config.server_url
    }

    /// 是否启用申请功能
    pub fn enable_apply(&self) -> bool {
        self.config.enable_apply && self.ip_geo.is_available()
    }

    /// 获取可信代理列表
    pub fn trusted_proxies(&self) -> Option<&Vec<IpRange>> {
        self.config.trusted_proxies.as_ref()
    }

    // === 安全相关方法 ===

    /// 验证管理员密码
    pub fn verify_admin(&self, password: &str) -> bool {
        self.config.admin_password == password
    }

    // === 服务访问 ===

    /// 获取 IP 地理位置服务
    pub fn ip_geo(&self) -> Arc<IpGeoService> {
        self.ip_geo.clone()
    }

    /// 获取侧边栏机器列表
    pub async fn get_sidebar_machines(&self) -> Vec<MachineListItem> {
        self.machine_service()
            .find_all_for_list()
            .await
            .unwrap_or_default()
    }

    // === Service 便捷方法 ===

    /// 获取 MachineService
    pub fn machine_service(&self) -> MachineService<'_, DatabaseConnection> {
        MachineService::new(&self.db)
    }

    /// 获取 TargetService
    pub fn target_service(&self) -> TargetService<'_, DatabaseConnection> {
        TargetService::new(&self.db)
    }

    /// 获取 PingService
    pub fn ping_service(&self) -> PingService<'_, DatabaseConnection> {
        PingService::new(&self.db)
    }

    /// 获取 ApplicationService
    pub fn application_service(&self) -> ApplicationService<'_, DatabaseConnection> {
        ApplicationService::new(&self.db, self.ip_geo.clone())
    }
}
