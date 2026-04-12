use thiserror::Error;

/// Service 层统一错误类型
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("IP geo service error: {0}")]
    IpGeo(String),

    #[error("Application error: {0}")]
    Application(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl ServiceError {
    /// 快速创建 NotFound 错误
    pub fn not_found(resource: impl AsRef<str>, id: impl std::fmt::Display) -> Self {
        Self::NotFound(format!("{} {} not found", resource.as_ref(), id))
    }

    /// 快速创建 Validation 错误
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// 快速创建 Conflict 错误
    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::Conflict(msg.into())
    }
}
