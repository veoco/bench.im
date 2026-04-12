use askama::Template;
use axum::{
    extract::State,
    http::{header::SET_COOKIE, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

use crate::{
    extractors::AdminUserWeb,
    templates::{AdminIndexTemplate, AdminLoginTemplate, AdminMachine, AdminTarget},
    AppState,
};


pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/login", get(admin_login_page).post(admin_login))
        .route("/admin/logout", post(admin_logout))
        .route("/admin/", get(admin_index_page))
}

#[derive(Debug, Validate, Deserialize)]
struct LoginRequest {
    #[validate(length(min = 1, max = 256))]
    password: String,
}

async fn admin_login_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let template = AdminLoginTemplate {
        site_name: state.site_name().to_string(),
        machines: state.get_sidebar_machines().await,
        current_machine_id: 0,
        enable_apply: state.enable_apply(),
        is_admin: true,
    };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

/// 处理登录请求，验证密码并设置 HttpOnly Cookie
async fn admin_login(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<LoginRequest>,
) -> Response {
    if !state.verify_admin(&req.password) {
        return (StatusCode::UNAUTHORIZED, "Invalid password").into_response();
    }

    // 检查是否 HTTPS（常用反向代理默认设置）
    let is_https = headers
        .get("x-forwarded-proto")
        .and_then(|v| v.to_str().ok())
        == Some("https")
        || headers
            .get("x-forwarded-scheme")
            .and_then(|v| v.to_str().ok())
            == Some("https");

    let secure_flag = if is_https { "Secure; " } else { "" };

    // 设置 HttpOnly Cookie
    // Path=/ 确保 /admin/* 和 /api/admin/* 都能访问此 Cookie
    let cookie = format!(
        "admin_token={}; Path=/; HttpOnly; SameSite=Strict; {}",
        req.password, secure_flag
    );

    (
        StatusCode::OK,
        [(SET_COOKIE, cookie)],
        "Login successful",
    )
        .into_response()
}

/// 处理登出请求，清除 Cookie
async fn admin_logout(headers: HeaderMap) -> Response {
    // 检查是否 HTTPS（常用反向代理默认设置）
    let is_https = headers
        .get("x-forwarded-proto")
        .and_then(|v| v.to_str().ok())
        == Some("https")
        || headers
            .get("x-forwarded-scheme")
            .and_then(|v| v.to_str().ok())
            == Some("https");

    let secure_flag = if is_https { "Secure; " } else { "" };

    // 设置过期时间为过去的日期来清除 Cookie
    // Path=/ 与设置时保持一致，才能正确清除
    let cookie = format!(
        "admin_token=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Strict; {}",
        secure_flag
    );

    (
        StatusCode::OK,
        [(SET_COOKIE, cookie)],
        "Logout successful",
    )
        .into_response()
}

async fn admin_index_page(
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    // 查询 admin 机器列表（完整信息）
    let admin_machines = match state.machine_service().find_all_admin().await {
        Ok(list) => list
            .into_iter()
            .map(|m| AdminMachine {
                id: m.id,
                name: m.name,
                ip: m.ip,
            })
            .collect(),
        Err(_) => vec![],
    };

    // 查询 admin 目标列表（完整信息）
    let admin_targets = match state.target_service().find_all_admin().await {
        Ok(list) => list
            .into_iter()
            .map(|t| AdminTarget {
                id: t.id,
                name: t.name,
                domain: t.domain.unwrap_or_default(),
                ipv4: t.ipv4.unwrap_or_default(),
                ipv6: t.ipv6.unwrap_or_default(),
            })
            .collect(),
        Err(_) => vec![],
    };

    let template = AdminIndexTemplate {
        site_name: state.site_name().to_string(),
        machines: state.get_sidebar_machines().await,
        current_machine_id: 0,
        admin_machines,
        admin_targets,
        enable_apply: state.enable_apply(),
        is_admin: true,
    };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}
