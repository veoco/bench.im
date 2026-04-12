use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use axum_valid::Valid;
use serde_json::{json, Value};

use crate::extractors::{ApiClient, ClientIp};
use crate::{ApiError, AppState};
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
    ClientIp(client_ip): ClientIp,
    Path(tid): Path<i32>,
    Valid(Json(ping_create)): Valid<Json<PingCreate>>,
) -> Result<Json<Value>, ApiError> {
    // 验证 target 存在
    let target = QueryCore::find_target_by_id(&state.db(), tid)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Target {} not found", tid)))?;

    // 创建 ping 记录
    MutationCore::create_ping(&state.db(), ping_create, machine.id, target.id).await?;

    // 更新 machine 和 target 的更新时间
    let _ = MutationCore::update_machine(&state.db(), machine.id, client_ip).await;
    let _ = MutationCore::update_target(&state.db(), target.id).await;

    Ok(Json(json!({"msg": "success"})))
}

pub async fn list_pings(
    State(state): State<Arc<AppState>>,
    Path((mid, tid, delta)): Path<(i32, i32, String)>,
    Query(form): Query<PingFilter>,
) -> Result<Json<Value>, ApiError> {
    let ipv6 = form.ipv6.unwrap_or(false);

    // 验证 machine 和 target 存在（使用并行查询优化性能）
    let (machine, target) = tokio::try_join!(
        QueryCore::find_machine_by_id(&state.db(), mid),
        QueryCore::find_target_by_id(&state.db(), tid)
    )?;

    let _ = machine.ok_or_else(|| ApiError::NotFound(format!("Machine {} not found", mid)))?;
    let _ = target.ok_or_else(|| ApiError::NotFound(format!("Target {} not found", tid)))?;

    // 查询 ping 数据
    let pings = QueryCore::find_pings_by_machine_id_and_target_id(
        &state.db(), mid, tid, &delta, ipv6,
    ).await?;

    let outputs: Vec<_> = pings
        .into_iter()
        .map(|p| (p.created.and_utc().timestamp(), p.min, p.avg, p.fail))
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
