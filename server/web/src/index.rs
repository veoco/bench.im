use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::{
    templates::{IndexTemplate, Machine, MachineTemplate, MachineForList, Target},
    AppState,
};
use server_service::query::Query;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index_page))
        .route("/m/{mid}", get(machine_page))
}

pub async fn fetch_machines_for_list(state: &Arc<AppState>) -> Vec<MachineForList> {
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

async fn index_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let targets: Vec<Target> = match Query::find_targets(&state.conn).await {
        Ok(list) => {
            let target_ids: Vec<i32> = list.iter().map(|t| t.id).collect();
            let latest_pings = Query::find_latest_pings_for_all_targets(&state.conn, target_ids)
                .await
                .unwrap_or_default();
            
            list.into_iter().map(|t| {
                let updated = latest_pings.get(&t.id)
                    .map(|dt| dt.and_utc().timestamp())
                    .unwrap_or(0);
                Target {
                    id: t.id,
                    name: t.name,
                    updated,
                }
            }).collect()
        }
        Err(_) => vec![],
    };

    let machines = fetch_machines_for_list(&state).await;

    let template = IndexTemplate { targets, machines };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn machine_page(
    Path(mid): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let machine_result = Query::find_machine_by_id(&state.conn, mid).await;
    let machine = match machine_result {
        Ok(Some(m)) => Machine {
            id: m.id,
            name: m.name,
            ip: m.ip,
        },
        _ => {
            return Html("Machine not found".to_string());
        }
    };

    let targets: Vec<Target> = match Query::find_targets_by_machine_id(&state.conn, mid).await {
        Ok(list) => {
            let target_ids: Vec<i32> = list.iter().map(|t| t.id).collect();
            let latest_pings = Query::find_latest_pings_for_machine_targets(&state.conn, mid, target_ids)
                .await
                .unwrap_or_default();
            
            list.into_iter().map(|t| {
                let updated = latest_pings.get(&t.id)
                    .map(|dt| dt.and_utc().timestamp())
                    .unwrap_or(0);
                Target {
                    id: t.id,
                    name: t.name,
                    updated,
                }
            }).collect()
        }
        Err(_) => vec![],
    };

    let machines = fetch_machines_for_list(&state).await;

    let template = MachineTemplate { machine, targets, machines };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}
