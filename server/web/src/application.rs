use std::sync::Arc;

use axum::{
    extract::State,
    response::Html,
    routing::get,
    Router,
};
use askama::Template;

use crate::{
    extractors::ClientIp,
    index::fetch_machines_for_list,
    templates::MachineForList,
    AppState,
};
use server_service::{ApplicationService, ApplyRequest, CommandConfig, ip_geo::parse_ip};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/apply", get(apply_page).post(apply_submit))
}

/// GET /apply - 显示申请页面
async fn apply_page(
    State(state): State<Arc<AppState>>,
    ClientIp(client_ip): ClientIp,
) -> Html<String> {
    let machines = fetch_machines_for_list(&state).await;

    // 如果功能未开启，显示关闭页面
    if !state.enable_apply {
        let template = ApplyDisabledTemplate {
            site_name: state.site_name.clone(),
            machines,
            current_machine_id: 0,
            enable_apply: state.enable_apply,
            is_admin: false,
        };
        return Html(template.render().unwrap_or_else(|_| "Template error".to_string()));
    }

    match ApplicationService::check_eligibility(&state.conn, &client_ip).await {
        Ok((province, isp, count)) => {
            // 符合条件，显示确认页面
            let template = ApplyTemplate {
                site_name: state.site_name.clone(),
                eligible: true,
                ip: client_ip,
                province: province.clone(),
                isp: isp.clone(),
                reason: String::new(),
                current_count: count,
                max_count: 3,
                machines,
                current_machine_id: 0,
                enable_apply: state.enable_apply,
                is_admin: false,
            };
            Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
        }
        Err(e) => {
            // 尝试解析 IP 获取省份和运营商信息
            let (province, isp) = parse_ip(&client_ip)
                .map(|geo| (geo.province, geo.isp))
                .unwrap_or((String::new(), String::new()));

            // 不符合条件，显示错误页面
            let template = ApplyTemplate {
                site_name: state.site_name.clone(),
                eligible: false,
                ip: client_ip,
                province,
                isp,
                reason: e.to_string(),
                current_count: 0,
                max_count: 3,
                machines,
                current_machine_id: 0,
                enable_apply: state.enable_apply,
                is_admin: false,
            };
            Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
        }
    }
}

/// POST /apply - 提交申请
async fn apply_submit(
    State(state): State<Arc<AppState>>,
    ClientIp(client_ip): ClientIp,
) -> Html<String> {
    let machines = fetch_machines_for_list(&state).await;

    // 如果功能未开启
    if !state.enable_apply {
        let template = ApplyDisabledTemplate {
            site_name: state.site_name.clone(),
            machines,
            current_machine_id: 0,
            enable_apply: state.enable_apply,
            is_admin: false,
        };
        return Html(template.render().unwrap_or_else(|_| "Template error".to_string()));
    }

    // 重新检查资格（防止并发问题）
    let (province, isp) = match ApplicationService::check_eligibility(&state.conn, &client_ip).await {
        Ok((prov, isp, _)) => (prov, isp),
        Err(e) => {
            // 尝试解析 IP 获取省份和运营商信息
            let (province, isp) = parse_ip(&client_ip)
                .map(|geo| (geo.province, geo.isp))
                .unwrap_or((String::new(), String::new()));

            let template = ApplyTemplate {
                site_name: state.site_name.clone(),
                eligible: false,
                ip: client_ip,
                province,
                isp,
                reason: e.to_string(),
                current_count: 0,
                max_count: 3,
                machines,
                current_machine_id: 0,
                enable_apply: state.enable_apply,
                is_admin: false,
            };
            return Html(template.render().unwrap_or_else(|_| "Template error".to_string()));
        }
    };

    // 提交申请
    let config = CommandConfig {
        server_url: &state.server_url,
    };
    let result = match ApplicationService::submit_application(
        &state.conn,
        ApplyRequest {
            ip: client_ip.clone(),
            province,
            isp,
        },
        config,
    ).await {
        Ok(result) => result,
        Err(e) => {
            let template = ApplyTemplate {
                site_name: state.site_name.clone(),
                eligible: false,
                ip: client_ip,
                province: String::new(),
                isp: String::new(),
                reason: e.to_string(),
                current_count: 0,
                max_count: 3,
                machines,
                current_machine_id: 0,
                enable_apply: state.enable_apply,
                is_admin: false,
            };
            return Html(template.render().unwrap_or_else(|_| "Template error".to_string()));
        }
    };

    // 显示成功页面
    let template = ApplySuccessTemplate {
        site_name: state.site_name.clone(),
        machine_id: result.id,
        name: result.name,
        key: result.key,
        command: result.command,
        machines,
        current_machine_id: 0,
        enable_apply: state.enable_apply,
        is_admin: false,
    };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

/// 申请页面模板
#[derive(Template)]
#[template(path = "apply/index.html")]
struct ApplyTemplate {
    site_name: String,
    eligible: bool,
    ip: String,
    province: String,
    isp: String,
    reason: String,
    current_count: i32,
    max_count: i32,
    machines: Vec<MachineForList>,
    current_machine_id: i32,
    enable_apply: bool,
    is_admin: bool,
}

/// 申请成功页面模板
#[derive(Template)]
#[template(path = "apply/success.html")]
struct ApplySuccessTemplate {
    site_name: String,
    machine_id: i32,
    name: String,
    key: String,
    command: String,
    machines: Vec<MachineForList>,
    current_machine_id: i32,
    enable_apply: bool,
    is_admin: bool,
}

/// 申请功能关闭页面模板
#[derive(Template)]
#[template(path = "apply/disabled.html")]
struct ApplyDisabledTemplate {
    site_name: String,
    machines: Vec<MachineForList>,
    current_machine_id: i32,
    enable_apply: bool,
    is_admin: bool,
}
