use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::{
    templates::{IndexTemplate, Machine, MachineTemplate, Target, TargetTemplate},
    AppState,
};


pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index_page))
        .route("/m/{mid}", get(machine_page))
        .route("/t/{tid}", get(target_page))
}

async fn index_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let targets: Vec<Target> = match state.target_service().find_all().await {
        Ok(list) => list
            .into_iter()
            .map(|t| Target {
                id: t.id,
                name: t.name,
                updated: t.updated.unwrap_or(0) as i64,
            })
            .collect(),
        Err(_) => vec![],
    };

    let template = IndexTemplate {
        site_name: state.site_name().to_string(),
        targets,
        machines: state.get_sidebar_machines().await,
        current_machine_id: 0,
        enable_apply: state.enable_apply(),
        is_admin: false,
    };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn machine_page(
    Path(mid): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let machine = match state.machine_service().find_by_id(mid).await {
        Ok(Some(m)) => Machine {
            id: m.id,
            name: m.name,
            ip: m.ip,
        },
        _ => {
            return Html("Machine not found".to_string());
        }
    };

    let targets: Vec<Target> = match state.target_service().find_all().await {
        Ok(list) => list
            .into_iter()
            .map(|t| Target {
                id: t.id,
                name: t.name,
                updated: t.updated.unwrap_or(0) as i64,
            })
            .collect(),
        Err(_) => vec![],
    };

    let template = MachineTemplate {
        site_name: state.site_name().to_string(),
        machine: machine.clone(),
        targets,
        machines: state.get_sidebar_machines().await,
        current_machine_id: machine.id,
        enable_apply: state.enable_apply(),
        is_admin: false,
    };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn target_page(
    Path(tid): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let target = match state.target_service().find_by_id(tid).await {
        Ok(Some(t)) => Target {
            id: t.id,
            name: t.name,
            updated: t.updated.unwrap_or(0) as i64,
        },
        _ => {
            return Html("Target not found".to_string());
        }
    };

    let machines: Vec<Machine> = match state.machine_service().find_all().await {
        Ok(list) => list
            .into_iter()
            .map(|m| Machine {
                id: m.id,
                name: m.name,
                ip: m.ip,
            })
            .collect(),
        Err(_) => vec![],
    };

    let template = TargetTemplate {
        site_name: state.site_name().to_string(),
        target: target.clone(),
        machines: state.get_sidebar_machines().await, // 用于侧边栏
        target_machines: machines,                    // 用于图表
        current_machine_id: 0,
        enable_apply: state.enable_apply(),
        is_admin: false,
    };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}
