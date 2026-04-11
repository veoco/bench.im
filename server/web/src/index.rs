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
use server_service::{query::Query, MachinePublic};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index_page))
        .route("/m/{mid}", get(machine_page))
        .route("/t/{tid}", get(target_page))
}

async fn index_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let targets: Vec<Target> = match Query::find_targets(&state.conn).await {
        Ok(list) => {
            list.into_iter().map(|t| Target {
                id: t.id,
                name: t.name,
                updated: 0,
            }).collect()
        }
        Err(_) => vec![],
    };

    let machines = Query::fetch_machines_for_list(&state.conn).await.unwrap_or_default();

    let template = IndexTemplate { site_name: state.site_name.clone(), targets, machines, current_machine_id: 0, enable_apply: state.enable_apply, is_admin: false };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn machine_page(
    Path(mid): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let machine_result = Query::find_machine_by_id(&state.conn, mid).await;
    let machine = match machine_result {
        Ok(Some(m)) => {
            // 使用 MachinePublic 的模糊处理逻辑对 IP 进行脱敏
            let machine_public = MachinePublic::from(m);
            Machine {
                id: machine_public.id,
                name: machine_public.name,
                ip: machine_public.ip,
            }
        }
        _ => {
            return Html("Machine not found".to_string());
        }
    };

    let targets: Vec<Target> = match Query::find_targets_by_machine_id(&state.conn, mid).await {
        Ok(list) => {
            list.into_iter().map(|t| Target {
                id: t.id,
                name: t.name,
                updated: 0,
            }).collect()
        }
        Err(_) => vec![],
    };

    let machines = Query::fetch_machines_for_list(&state.conn).await.unwrap_or_default();

    let template = MachineTemplate { site_name: state.site_name.clone(), machine: machine.clone(), targets, machines, current_machine_id: machine.id, enable_apply: state.enable_apply, is_admin: false };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn target_page(
    Path(tid): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let target_result = Query::find_target_by_id(&state.conn, tid).await;
    let target = match target_result {
        Ok(Some(t)) => Target {
            id: t.id,
            name: t.name,
            updated: t.updated.map(|dt| dt.and_utc().timestamp()).unwrap_or(0),
        },
        _ => {
            return Html("Target not found".to_string());
        }
    };

    let machines: Vec<Machine> = match Query::find_machines(&state.conn).await {
        Ok(list) => {
            list.into_iter().map(|m| {
                Machine {
                    id: m.id,
                    name: m.name,
                    ip: m.ip,
                }
            }).collect()
        }
        Err(_) => vec![],
    };

    let machines_for_list = Query::fetch_machines_for_list(&state.conn).await.unwrap_or_default();

    let template = TargetTemplate {
        site_name: state.site_name.clone(),
        target: target.clone(),
        machines: machines_for_list.clone(),  // 用于侧边栏
        target_machines: machines,            // 用于图表
        current_machine_id: 0,
        enable_apply: state.enable_apply,
        is_admin: false,
    };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}
