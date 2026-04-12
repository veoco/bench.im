use std::sync::Arc;

use server_service::sea_orm::DatabaseConnection;

use crate::config::Config;
use crate::IpRange;

/// 应用状态 - 封装配置和数据库连接
#[derive(Clone)]
pub struct AppState {
    /// 数据库连接 - 私有，强制通过 service 层访问
    db: DatabaseConnection,
    /// 配置 - 使用 Arc 共享
    config: Arc<Config>,
}

impl AppState {
    /// 创建新的 AppState
    pub fn new(db: DatabaseConnection, config: Config) -> Self {
        Self {
            db,
            config: Arc::new(config),
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
        self.config.enable_apply
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

    // === 数据库访问 ===

    /// 获取数据库连接（仅同 crate 可访问）
    pub(crate) fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}
