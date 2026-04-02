use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::{ConnectInfo, Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_valid::Valid;
use serde_json::{json, Value};

use crate::extractors::{ApiClient, ClientIp};
use crate::AppState;
use server_service::{Mutation as MutationCore, PingCreate, PingFilter, Query as QueryCore};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/client/targets/{tid}", post(create_ping_client))
        .route("/api/machines/{mid}/targets/{tid}/{delta}", get(list_pings))
        .route("/api/targets/{tid}/machines/{mid}/{delta}", get(list_pings_by_target))
}

pub async fn create_ping_client(
    State(state): State<Arc<AppState>>,
    ApiClient(machine): ApiClient,
    ClientIp(ip): ClientIp,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(tid): Path<i32>,
    Valid(Json(ping_create)): Valid<Json<PingCreate>>,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    let client_ip = if ip.is_empty() {
        addr.ip().to_string()
    } else {
        ip
    };

    if let Ok(Some(machine)) = QueryCore::find_machine_by_id(&state.conn, machine.id).await {
        if let Ok(Some(target)) = QueryCore::find_target_by_id(&state.conn, tid).await {
            if let Ok(_) =
                MutationCore::create_ping(&state.conn, ping_create, machine.id, target.id).await
            {
                let _ = MutationCore::update_machine(&state.conn, machine.id, client_ip).await;
                let _ = MutationCore::update_target(&state.conn, target.id).await;
                res = json!({"msg": "success"});
                status = StatusCode::OK;
            }
        }
    }
    (status, Json(res))
}

pub async fn list_pings(
    State(state): State<Arc<AppState>>,
    Path((mid, tid, delta)): Path<(i32, i32, String)>,
    Query(form): Query<PingFilter>,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    let ipv6 = form.ipv6.unwrap_or(false);

    if let Ok(Some(machine)) = QueryCore::find_machine_by_id(&state.conn, mid).await {
        if let Ok(Some(target)) = QueryCore::find_target_by_id(&state.conn, tid).await {
            if let Ok(pings) = QueryCore::find_pings_by_machine_id_and_target_id(
                &state.conn,
                machine.id,
                target.id,
                &delta,
                ipv6,
            )
            .await
            {
                let mut outputs = vec![];
                for p in pings {
                    outputs.push((p.created.and_utc().timestamp(), p.min, p.avg, p.fail));
                }
                res = json!({
                    "results": outputs,
                });
                status = StatusCode::OK;
            }
        }
    }
    (status, Json(res))
}

pub async fn list_pings_by_target(
    State(state): State<Arc<AppState>>,
    Path((tid, mid, delta)): Path<(i32, i32, String)>,
    Query(form): Query<PingFilter>,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    let ipv6 = form.ipv6.unwrap_or(false);

    if let Ok(Some(target)) = QueryCore::find_target_by_id(&state.conn, tid).await {
        if let Ok(Some(machine)) = QueryCore::find_machine_by_id(&state.conn, mid).await {
            if let Ok(pings) = QueryCore::find_pings_by_machine_id_and_target_id(
                &state.conn,
                machine.id,
                target.id,
                &delta,
                ipv6,
            )
            .await
            {
                let mut outputs = vec![];
                for p in pings {
                    outputs.push((p.created.and_utc().timestamp(), p.min, p.avg, p.fail));
                }
                res = json!({
                    "results": outputs,
                });
                status = StatusCode::OK;
            }
        }
    }
    (status, Json(res))
}
