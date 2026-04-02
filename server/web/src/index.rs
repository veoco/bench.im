use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::{
    templates::{IndexTemplate, Machine, MachineTemplate, MachineForList, Target, TargetWithChartData},
    AppState,
};
use server_service::{query::Query, MachinePublic};

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

fn calculate_status_color(updated: i64) -> String {
    if updated == 0 {
        return "gray".to_string();
    }
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let diff = current_time - updated;
    if diff < 5 * 60 {
        "green".to_string()
    } else if diff < 10 * 60 {
        "yellow".to_string()
    } else {
        "red".to_string()
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
                let status_color = calculate_status_color(updated);
                Target {
                    id: t.id,
                    name: t.name,
                    updated,
                    status_color,
                }
            }).collect()
        }
        Err(_) => vec![],
    };

    let machines = fetch_machines_for_list(&state).await;

    let template = IndexTemplate { site_name: state.site_name.clone(), targets, machines, current_machine_id: 0 };
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

    let targets: Vec<TargetWithChartData> = match Query::find_targets_by_machine_id(&state.conn, mid).await {
        Ok(list) => {
            let target_ids: Vec<i32> = list.iter().map(|t| t.id).collect();
            
            // 批量查询：一次性获取所有目标的最新 ping 时间和图表数据
            let (latest_pings, chart_data_map) = tokio::join!(
                Query::find_latest_pings_for_machine_targets(&state.conn, mid, target_ids.clone()),
                Query::find_pings_for_machine_targets(&state.conn, mid, target_ids, "24h", false)
            );
            
            let latest_pings = latest_pings.unwrap_or_default();
            let chart_data_map = chart_data_map.unwrap_or_default();
            
            // 构建目标数据（无需单独查询）
            list.into_iter().map(|t| {
                let updated = latest_pings.get(&t.id)
                    .map(|dt| dt.and_utc().timestamp())
                    .unwrap_or(0);
                let status_color = calculate_status_color(updated);
                
                // 从批量查询结果中获取图表数据
                let chart_data = chart_data_map.get(&t.id)
                    .map(|pings| pings.iter().map(|p| {
                        (
                            p.created.and_utc().timestamp(),
                            p.min as f32,
                            p.avg as f32,
                            p.fail,
                        )
                    }).collect())
                    .unwrap_or_default();
                
                TargetWithChartData {
                    id: t.id,
                    name: t.name,
                    updated,
                    status_color,
                    chart_data,
                }
            }).collect()
        }
        Err(_) => vec![],
    };

    let machines = fetch_machines_for_list(&state).await;

    let template = MachineTemplate { site_name: state.site_name.clone(), machine: machine.clone(), targets, machines, current_machine_id: machine.id };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}
