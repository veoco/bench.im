use std::future::Future;
use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::Html,
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde_json::{json, Value};

use crate::AppState;
use entity::machine::Model as Machine;
use server_service::Query as QueryCore;

pub struct ClientIp(pub String);

impl<S> FromRequestParts<S> for ClientIp
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // 提取 IP 并去除端口号
            let extract_ip = |value: &str| -> Option<String> {
                let ip = value.trim();
                if ip.is_empty() {
                    return None;
                }

                // IPv6 地址处理
                if ip.starts_with('[') {
                    // IPv6 带端口格式: [2408:...]:port
                    // 提取方括号内的内容
                    if let Some(end) = ip.find(']') {
                        return Some(ip[1..end].to_string());
                    }
                } else if ip.contains(':') && !ip.contains('.') {
                    // 纯 IPv6 地址（包含冒号但不包含点）
                    return Some(ip.to_string());
                }

                // IPv4 地址处理（去除端口号）
                // IPv4:port 格式，如 1.2.3.4:12345
                let ip = ip.split(':').next().unwrap_or(ip);
                Some(ip.to_string())
            };

            if let Some(header_value) = parts.headers.get("x-forwarded-for") {
                if let Ok(value) = header_value.to_str() {
                    if let Some(ip) = value.split(',').last() {
                        if let Some(ip) = extract_ip(ip) {
                            return Ok(ClientIp(ip));
                        }
                    }
                }
            }

            if let Some(header_value) = parts.headers.get("x-real-ip") {
                if let Ok(value) = header_value.to_str() {
                    if let Some(ip) = extract_ip(value) {
                        return Ok(ClientIp(ip));
                    }
                }
            }

            Ok(ClientIp(String::new()))
        }
    }
}

/// 辅助函数：从请求中提取指定名称的 Cookie 值
fn extract_cookie(parts: &Parts, name: &str) -> Option<String> {
    parts
        .headers
        .get_all("Cookie")
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .map(|v| v.trim())
        .filter_map(|v| v.strip_prefix(&format!("{}=", name)))
        .next()
        .map(|v| v.to_string())
}

/// 用于 Admin API 的认证提取器
/// 支持 Cookie 或 Bearer Token，优先 Cookie
pub struct AdminAuth;

impl<S> FromRequestParts<S> for AdminAuth
where
    S: Send + Sync,
    Arc<AppState>: FromRef<S>,
{
    type Rejection = (StatusCode, Json<Value>);

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let s = Arc::from_ref(state);

            // 1. 优先从 Cookie 读取
            if let Some(token) = extract_cookie(parts, "admin_token") {
                if token == s.admin_password {
                    return Ok(Self);
                }
            }

            // 2. 其次从 Bearer Token 读取
            if let Ok(TypedHeader(Authorization(bearer))) =
                parts.extract::<TypedHeader<Authorization<Bearer>>>().await
            {
                let token = bearer.token();
                if token == s.admin_password {
                    return Ok(Self);
                }
            }

            Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"msg": "Login required"})),
            ))
        }
    }
}

/// 用于 HTML 页面路由的管理员认证提取器
/// 仅从 Cookie 中读取 admin_token 进行认证
/// 认证失败时返回 401，由前端处理重定向
pub struct AdminUserWeb;

impl<S> FromRequestParts<S> for AdminUserWeb
where
    S: Send + Sync,
    Arc<AppState>: FromRef<S>,
{
    type Rejection = (StatusCode, Html<String>);

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let s = Arc::from_ref(state);

            // 从 Cookie 中读取 admin_token
            if let Some(token) = extract_cookie(parts, "admin_token") {
                if token == s.admin_password {
                    return Ok(Self);
                }
            }

            Err((
                StatusCode::UNAUTHORIZED,
                Html(String::from(r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><title>Unauthorized</title><script>window.location.href='/admin/login';</script></head><body></body></html>"#)),
            ))
        }
    }
}

pub struct ApiClient(pub Machine);

impl<S> FromRequestParts<S> for ApiClient
where
    S: Send + Sync,
    Arc<AppState>: FromRef<S>,
{
    type Rejection = (StatusCode, Json<Value>);

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let s = Arc::from_ref(state);
            if let Ok(TypedHeader(Authorization(bearer))) =
                parts.extract::<TypedHeader<Authorization<Bearer>>>().await
            {
                let token = bearer.token();
                let (mid, key) = token.split_once(':').ok_or((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"msg": "Invalid API token format"})),
                ))?;
                if let Ok(Some(machine)) =
                    QueryCore::find_machine_by_id(&s.conn, mid.parse::<i32>().unwrap_or(0)).await
                {
                    if machine.key == key {
                        return Ok(Self(machine));
                    }
                }
            }
            Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"msg": "Api token required"})),
            ))
        }
    }
}
