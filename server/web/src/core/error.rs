use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

/// API 错误类型
#[derive(Debug)]
pub enum ApiError {
    /// 资源未找到
    NotFound(String),
    /// 数据库错误（内部错误，不暴露给客户端）
    DatabaseError(String),
    /// 验证错误
    ValidationError(String),
    /// 未授权
    Unauthorized,
    /// 资源冲突（如重复创建）
    Conflict(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::DatabaseError(msg) => {
                // 记录详细错误，但返回模糊信息给客户端
                tracing::error!("Database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
        };

        (status, Json(json!({ "msg": msg }))).into_response()
    }
}

impl From<sea_orm::DbErr> for ApiError {
    fn from(e: sea_orm::DbErr) -> Self {
        ApiError::DatabaseError(e.to_string())
    }
}

impl From<server_service::ServiceError> for ApiError {
    fn from(e: server_service::ServiceError) -> Self {
        use server_service::ServiceError;
        match e {
            ServiceError::NotFound(msg) => ApiError::NotFound(msg),
            ServiceError::Validation(msg) => ApiError::ValidationError(msg),
            ServiceError::Database(e) => ApiError::DatabaseError(e.to_string()),
            ServiceError::Conflict(msg) => ApiError::Conflict(msg),
            ServiceError::IpGeo(msg) => ApiError::DatabaseError(msg),
            ServiceError::Application(msg) => ApiError::ValidationError(msg),
            ServiceError::Unauthorized(_msg) => ApiError::Unauthorized,
        }
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(e: validator::ValidationErrors) -> Self {
        ApiError::ValidationError(e.to_string())
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
        }
    }
}

/// 统一模板渲染函数
pub fn render_template<T: askama::Template>(template: T) -> Result<String, ApiError> {
    match template.render() {
        Ok(html) => Ok(html),
        Err(e) => {
            tracing::error!("Template render error: {}", e);
            // 开发环境返回详细错误
            if cfg!(debug_assertions) {
                Err(ApiError::DatabaseError(format!("Template error: {}", e)))
            } else {
                // 生产环境返回友好提示
                Ok("<h1>页面加载失败</h1><p>请稍后重试</p>".to_string())
            }
        }
    }
}
