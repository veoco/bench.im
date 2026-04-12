use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use axum_valid::Valid;
use serde_json::{json, Value};

use server_service::input::{CreatePingRequest, PingFilter};

use crate::extractors::{ApiClient, ClientIp};
use crate::{ApiError, AppState};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/client/targets/{tid}", post(create_ping_client))
        .route("/api/machines/{mid}/targets/{tid}/{delta}", get(list_pings))
        .route("/api/targets/{tid}/machines/{mid}/{delta}", get(list_pings_by_target))
}

pub async fn create_ping_client(
    State(state): State<Arc<AppState>>,
    ApiClient(machine): ApiClient,
    ClientIp(client_ip): ClientIp,
    Path(tid): Path<i32>,
    Valid(Json(req)): Valid<Json<CreatePingRequest>>,
) -> Result<Json<Value>, ApiError> {
    // 验证 target 存在
    let target = state
        .target_service()
        .find_by_id(tid)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Target {} not found", tid)))?;

    // 使用事务创建 ping 并更新相关记录
    state
        .ping_service()
        .create_with_updates(machine.id, target.id, req, client_ip)
        .await?;

    Ok(Json(json!({"msg": "success"})))
}

pub async fn list_pings(
    State(state): State<Arc<AppState>>,
    Path((mid, tid, delta)): Path<(i32, i32, String)>,
    Query(form): Query<PingFilter>,
) -> Result<Json<Value>, ApiError> {
    let ipv6 = form.ipv6.unwrap_or(false);

    // 验证 machine 和 target 存在（使用并行查询优化性能）
    // 由于 try_join! 需要引用，这里需要保留中间变量
    let machine_service = state.machine_service();
    let target_service = state.target_service();
    let (machine, target) = tokio::try_join!(
        machine_service.find_by_id(mid),
        target_service.find_by_id(tid)
    )?;

    let _ = machine.ok_or_else(|| ApiError::NotFound(format!("Machine {} not found", mid)))?;
    let _ = target.ok_or_else(|| ApiError::NotFound(format!("Target {} not found", tid)))?;

    // 查询 ping 数据
    let pings = state
        .ping_service()
        .find_by_machine_and_target(mid, tid, &delta, ipv6)
        .await?;

    let outputs: Vec<_> = pings
        .into_iter()
        .map(|p| (p.timestamp, p.min, p.avg, p.fail))
        .collect();

    Ok(Json(json!({"results": outputs})))
}

pub async fn list_pings_by_target(
    State(state): State<Arc<AppState>>,
    Path((tid, mid, delta)): Path<(i32, i32, String)>,
    Query(form): Query<PingFilter>,
) -> Result<Json<Value>, ApiError> {
    // 与 list_pings 逻辑相同，复用实现
    list_pings(State(state), Path((mid, tid, delta)), Query(form)).await
}
