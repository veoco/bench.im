use std::future::Future;
use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
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
                // 去除端口号（IPv4:port 格式，如 1.2.3.4:12345）
                // 注意：IPv6 地址格式复杂，这里简单处理 IPv4 带端口的情况
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

pub struct AdminUser {}

impl<S> FromRequestParts<S> for AdminUser
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
                if token == s.admin_password {
                    return Ok(Self {});
                }
            }
            Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"msg": "Login required"})),
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
