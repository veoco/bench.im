use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::{ConnectInfo, FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::Html,
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde_json::{json, Value};

use crate::{AppState, is_trusted_proxy};
use entity::machine::Model as Machine;
use server_service::Query as QueryCore;

pub struct ClientIp(pub String);

impl<S> FromRequestParts<S> for ClientIp
where
    S: Send + Sync,
    Arc<AppState>: FromRef<S>,
{
    type Rejection = std::convert::Infallible;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let s = Arc::from_ref(state);

            // 获取连接地址（始终可信）
            let connect_info: Option<ConnectInfo<SocketAddr>> = parts.extensions.get::<ConnectInfo<SocketAddr>>().copied();
            let peer_addr = connect_info.map(|ci| ci.0.ip());

            // 从 Header 中提取 IP 的辅助函数
            let extract_ip_from_header = |value: &str| -> Option<String> {
                let ip = value.trim();
                if ip.is_empty() {
                    return None;
                }

                // IPv6 地址处理
                if ip.starts_with('[') {
                    if let Some(end) = ip.find(']') {
                        return Some(ip[1..end].to_string());
                    }
                } else if ip.contains(':') && !ip.contains('.') {
                    return Some(ip.to_string());
                }

                // IPv4 地址处理（去除端口号）
                let ip = ip.split(':').next().unwrap_or(ip);
                Some(ip.to_string())
            };

            // 尝试从 Header 获取 IP（X-Forwarded-For 优先）
            let header_ip = parts
                .headers
                .get("x-forwarded-for")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.split(',').last())
                .and_then(extract_ip_from_header)
                .or_else(|| {
                    parts
                        .headers
                        .get("x-real-ip")
                        .and_then(|v| v.to_str().ok())
                        .and_then(extract_ip_from_header)
                });

            // 决定使用哪个 IP
            let client_ip = if let Some(ref trusted) = s.trusted_proxies {
                // 配置了可信代理：只有来自可信代理的请求才使用 Header IP
                if let Some(ref peer) = peer_addr {
                    if is_trusted_proxy(peer, trusted) {
                        header_ip.unwrap_or_else(|| peer.to_string())
                    } else {
                        // 非可信来源：使用连接地址
                        peer.to_string()
                    }
                } else {
                    header_ip.unwrap_or_default()
                }
            } else {
                // 未配置可信代理：始终使用连接地址（最安全的默认行为）
                peer_addr.map(|p| p.to_string()).unwrap_or_default()
            };

            Ok(ClientIp(client_ip))
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
