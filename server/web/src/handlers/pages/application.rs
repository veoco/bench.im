use std::sync::Arc;

use axum::{
    extract::State,
    response::Html,
    routing::get,
    Router,
};

use server_service::input::CreateApplicationRequest;

use crate::core::{AppState, ClientIp};
use crate::templates::application::{ApplyDisabledTemplate, ApplySuccessTemplate, ApplyTemplate};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/apply", get(apply_page).post(apply_submit))
}

/// GET /apply - 显示申请页面
async fn apply_page(
    State(state): State<Arc<AppState>>,
    ClientIp(client_ip): ClientIp,
) -> Html<String> {
    use askama::Template;

    // 如果功能未开启，显示关闭页面
    if !state.enable_apply() {
        let template = ApplyDisabledTemplate {
            site_name: state.site_name().to_string(),
            machines: state.get_sidebar_machines().await,
            current_machine_id: 0,
            enable_apply: state.enable_apply(),
            is_admin: false,
        };
        return Html(template.render().unwrap_or_else(|_| "Template error".to_string()));
    }

    match state
        .application_service()
        .check_eligibility(&client_ip)
        .await
    {
        Ok((province, isp, count)) => {
            // 符合条件，显示确认页面
            let template = ApplyTemplate {
                site_name: state.site_name().to_string(),
                eligible: true,
                ip: client_ip,
                province: province.clone(),
                isp: isp.clone(),
                reason: String::new(),
                current_count: count,
                max_count: 3,
                machines: state.get_sidebar_machines().await,
                current_machine_id: 0,
                enable_apply: state.enable_apply(),
                is_admin: false,
            };
            Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
        }
        Err(e) => {
            // 尝试解析 IP 获取省份和运营商信息
            let (province, isp) = state
                .ip_geo()
                .parse_ip(&client_ip)
                .map(|geo| (geo.province, geo.isp))
                .unwrap_or((String::new(), String::new()));

            // 不符合条件，显示错误页面
            let template = ApplyTemplate {
                site_name: state.site_name().to_string(),
                eligible: false,
                ip: client_ip,
                province,
                isp,
                reason: e.to_string(),
                current_count: 0,
                max_count: 3,
                machines: state.get_sidebar_machines().await,
                current_machine_id: 0,
                enable_apply: state.enable_apply(),
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
    use askama::Template;

    // 如果功能未开启
    if !state.enable_apply() {
        let template = ApplyDisabledTemplate {
            site_name: state.site_name().to_string(),
            machines: state.get_sidebar_machines().await,
            current_machine_id: 0,
            enable_apply: state.enable_apply(),
            is_admin: false,
        };
        return Html(template.render().unwrap_or_else(|_| "Template error".to_string()));
    }

    // 重新检查资格（防止并发问题）
    let (province, isp) = match state
        .application_service()
        .check_eligibility(&client_ip)
        .await
    {
        Ok((prov, isp, _)) => (prov, isp),
        Err(e) => {
            // 尝试解析 IP 获取省份和运营商信息
            let (province, isp) = state
                .ip_geo()
                .parse_ip(&client_ip)
                .map(|geo| (geo.province, geo.isp))
                .unwrap_or((String::new(), String::new()));

            let template = ApplyTemplate {
                site_name: state.site_name().to_string(),
                eligible: false,
                ip: client_ip,
                province,
                isp,
                reason: e.to_string(),
                current_count: 0,
                max_count: 3,
                machines: state.get_sidebar_machines().await,
                current_machine_id: 0,
                enable_apply: state.enable_apply(),
                is_admin: false,
            };
            return Html(template.render().unwrap_or_else(|_| "Template error".to_string()));
        }
    };

    // 提交申请
    let result = match state
        .application_service()
        .submit(
            CreateApplicationRequest {
                ip: client_ip.clone(),
                province,
                isp,
            },
            state.server_url(),
        )
        .await
    {
        Ok(result) => result,
        Err(e) => {
            let template = ApplyTemplate {
                site_name: state.site_name().to_string(),
                eligible: false,
                ip: client_ip,
                province: String::new(),
                isp: String::new(),
                reason: e.to_string(),
                current_count: 0,
                max_count: 3,
                machines: state.get_sidebar_machines().await,
                current_machine_id: 0,
                enable_apply: state.enable_apply(),
                is_admin: false,
            };
            return Html(template.render().unwrap_or_else(|_| "Template error".to_string()));
        }
    };

    // 显示成功页面
    let template = ApplySuccessTemplate {
        site_name: state.site_name().to_string(),
        machine_id: result.id,
        name: result.name,
        key: result.key,
        command: result.command,
        machines: state.get_sidebar_machines().await,
        current_machine_id: 0,
        enable_apply: state.enable_apply(),
        is_admin: false,
    };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}


