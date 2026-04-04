use std::sync::Arc;
use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    response::Html,
    routing::get,
    Router,
};
use askama::Template;

use crate::{
    index::fetch_machines_for_list,
    templates::MachineForList,
    AppState,
};
use server_service::{ApplicationService, ApplyRequest};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/apply", get(apply_page).post(apply_submit))
}

/// GET /apply - 显示申请页面
async fn apply_page(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let machines = fetch_machines_for_list(&state).await;
    
    // 如果功能未开启，显示关闭页面
    if !state.enable_apply {
        let template = ApplyDisabledTemplate {
            site_name: state.site_name.clone(),
            machines,
            current_machine_id: 0,
            enable_apply: state.enable_apply,
        };
        return Html(template.render().unwrap_or_else(|_| "Template error".to_string()));
    }

    let client_ip = extract_client_ip(addr);

    match ApplicationService::check_eligibility(&state.conn, &client_ip).await {
        Ok((province, isp)) => {
            // 符合条件，显示确认页面
            let template = ApplyTemplate {
                site_name: state.site_name.clone(),
                eligible: true,
                ip: client_ip,
                province: province.clone(),
                isp: isp.clone(),
                reason: String::new(),
                machines,
                current_machine_id: 0,
                enable_apply: state.enable_apply,
            };
            Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
        }
        Err(e) => {
            // 不符合条件，显示错误页面
            let template = ApplyTemplate {
                site_name: state.site_name.clone(),
                eligible: false,
                ip: client_ip,
                province: String::new(),
                isp: String::new(),
                reason: e.to_string(),
                machines,
                current_machine_id: 0,
                enable_apply: state.enable_apply,
            };
            Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
        }
    }
}

/// POST /apply - 提交申请
async fn apply_submit(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Html<String> {
    let machines = fetch_machines_for_list(&state).await;
    
    // 如果功能未开启
    if !state.enable_apply {
        let template = ApplyDisabledTemplate {
            site_name: state.site_name.clone(),
            machines,
            current_machine_id: 0,
            enable_apply: state.enable_apply,
        };
        return Html(template.render().unwrap_or_else(|_| "Template error".to_string()));
    }

    let client_ip = extract_client_ip(addr);

    // 重新检查资格（防止并发问题）
    let (province, isp) = match ApplicationService::check_eligibility(&state.conn, &client_ip).await {
        Ok(info) => info,
        Err(e) => {
            let template = ApplyTemplate {
                site_name: state.site_name.clone(),
                eligible: false,
                ip: client_ip,
                province: String::new(),
                isp: String::new(),
                reason: e.to_string(),
                machines,
                current_machine_id: 0,
                enable_apply: state.enable_apply,
            };
            return Html(template.render().unwrap_or_else(|_| "Template error".to_string()));
        }
    };

    // 提交申请
    let result = match ApplicationService::submit_application(
        &state.conn,
        ApplyRequest {
            ip: client_ip,
            province,
            isp,
        }
    ).await {
        Ok(result) => result,
        Err(e) => {
            let template = ApplyTemplate {
                site_name: state.site_name.clone(),
                eligible: false,
                ip: String::new(),
                province: String::new(),
                isp: String::new(),
                reason: e.to_string(),
                machines,
                current_machine_id: 0,
                enable_apply: state.enable_apply,
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
    };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

/// 提取客户端真实 IP
fn extract_client_ip(addr: SocketAddr) -> String {
    // 注意：如果有反向代理，应该从 X-Forwarded-For 头部获取
    // 这里简化处理，直接使用连接 IP
    addr.ip().to_string()
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
    machines: Vec<MachineForList>,
    current_machine_id: i32,
    enable_apply: bool,
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
}

/// 申请功能关闭页面模板
#[derive(Template)]
#[template(path = "apply/disabled.html")]
struct ApplyDisabledTemplate {
    site_name: String,
    machines: Vec<MachineForList>,
    current_machine_id: i32,
    enable_apply: bool,
}
