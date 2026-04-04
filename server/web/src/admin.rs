use askama::Template;
use axum::{
    extract::State,
    response::Html,
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::{
    templates::{AdminIndexTemplate, AdminLoginTemplate, MachineForList, AdminMachine, AdminTarget},
    AppState,
};
use server_service::query::Query;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/login", get(admin_login_page))
        .route("/admin/", get(admin_index_page))
}

async fn fetch_machines_for_list(state: &Arc<AppState>) -> Vec<MachineForList> {
    match Query::find_machines(&state.conn).await {
        Ok(list) => list
            .into_iter()
            .map(|m| MachineForList {
                id: m.id,
                name: m.name,
                updated: m.updated.map(|dt| dt.and_utc().timestamp()).unwrap_or(0),
            })
            .collect(),
        Err(_) => vec![],
    }
}

async fn admin_login_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let machines = fetch_machines_for_list(&state).await;
    let template = AdminLoginTemplate { site_name: state.site_name.clone(), machines, current_machine_id: 0 };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn admin_index_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let machines = fetch_machines_for_list(&state).await;
    
    // 查询 admin 机器列表（完整信息）
    let admin_machines = match Query::find_machines(&state.conn).await {
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
    let admin_targets = match Query::find_targets(&state.conn).await {
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
        site_name: state.site_name.clone(),
        machines,
        current_machine_id: 0,
        admin_machines,
        admin_targets,
    };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}
