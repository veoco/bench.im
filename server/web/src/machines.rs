use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::get,
    Json, Router,
};
use axum_valid::Valid;
use serde_json::{json, Value};

use crate::extractors::{AdminAuth, AdminUserWeb};
use crate::{
    templates::{DeleteTemplate, EditMachineTemplate},
    ApiError, AppState, render_template,
};
use server_service::{
    MachineCreateAdmin, MachinePublic, MachineTargetsPublic, Mutation as MutationCore,
    Query as QueryCore, TargetPublic,
};
use entity::machine::Model as Machine;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        // API 路由
        .route("/api/machines/", get(list_machines))
        .route("/api/machines/{mid}", get(get_machine_by_mid))
        .route(
            "/api/admin/machines/",
            axum::routing::post(create_machine_admin).get(list_machines_admin),
        )
        .route(
            "/api/admin/machines/{mid}",
            get(get_machine_by_mid_admin)
                .post(edit_machine_admin)
                .delete(delete_machine_by_mid_admin),
        )
        // 页面路由
        .route("/admin/machines/new", get(new_machine_page))
        .route("/admin/machines/{mid}", get(edit_machine_page))
        .route("/admin/machines/{mid}/delete", get(delete_machine_page))
}

pub async fn list_machines(State(state): State<Arc<AppState>>) -> Result<Json<Vec<MachinePublic>>, ApiError> {
    let machines = QueryCore::find_machines(&state.conn).await?;
    Ok(Json(machines.into_iter().map(MachinePublic::from).collect()))
}

pub async fn get_machine_by_mid(
    State(state): State<Arc<AppState>>,
    Path(mid): Path<i32>,
) -> Result<Json<MachineTargetsPublic>, ApiError> {
    let machine = QueryCore::find_machine_by_id(&state.conn, mid)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Machine {} not found", mid)))?;

    let mut result = MachineTargetsPublic::from(MachinePublic::from(machine));

    // 获取所有 targets
    let targets = QueryCore::find_targets(&state.conn).await?;
    result.targets = targets.into_iter().map(TargetPublic::from).collect();

    Ok(Json(result))
}

pub async fn create_machine_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Valid(Json(machine_create)): Valid<Json<MachineCreateAdmin>>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    // 检查是否已存在
    if QueryCore::find_machine_by_name(&state.conn, &machine_create.name).await?.is_some() {
        return Err(ApiError::Conflict("Machine with this name already exists".to_string()));
    }

    let machine = MutationCore::create_machine(&state.conn, &machine_create).await?;
    let mid = machine.id.as_ref();

    Ok((StatusCode::CREATED, Json(json!({"msg": mid}))))
}

pub async fn edit_machine_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(mid): Path<i32>,
    Valid(Json(machine_create)): Valid<Json<MachineCreateAdmin>>,
) -> Result<Json<Value>, ApiError> {
    let machine = QueryCore::find_machine_by_id(&state.conn, mid)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Machine {} not found", mid)))?;

    MutationCore::edit_machine(&state.conn, machine.id, &machine_create).await?;
    Ok(Json(json!({"msg": "success"})))
}

pub async fn list_machines_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
) -> Result<Json<Vec<Machine>>, ApiError> {
    let machines = QueryCore::find_machines(&state.conn).await?;
    Ok(Json(machines))
}

pub async fn get_machine_by_mid_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(mid): Path<i32>,
) -> Result<Json<Machine>, ApiError> {
    let machine = QueryCore::find_machine_by_id(&state.conn, mid)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Machine {} not found", mid)))?;

    Ok(Json(machine))
}

pub async fn delete_machine_by_mid_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(mid): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    // 检查 machine 存在
    let _ = QueryCore::find_machine_by_id(&state.conn, mid)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Machine {} not found", mid)))?;

    MutationCore::delete_machine(&state.conn, mid).await?;
    Ok(Json(json!({"msg": "success"})))
}

// 页面 handlers
async fn new_machine_page(
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let machines = QueryCore::fetch_machines_for_list(&state.conn).await.unwrap_or_default();
    let template = EditMachineTemplate {
        site_name: state.site_name.clone(),
        is_edit: false,
        id: 0,
        name: "".to_string(),
        ip: "".to_string(),
        key: "".to_string(),
        machines,
        current_machine_id: 0,
        enable_apply: state.enable_apply,
        is_admin: true,
    };
    Html(render_template(template).unwrap_or_else(|e| e.to_string()))
}

async fn edit_machine_page(
    Path(mid): Path<i32>,
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let machine_result = QueryCore::find_machine_by_id(&state.conn, mid).await;
    let machines = QueryCore::fetch_machines_for_list(&state.conn).await.unwrap_or_default();

    let template = match machine_result {
        Ok(Some(m)) => EditMachineTemplate {
            site_name: state.site_name.clone(),
            is_edit: true,
            id: m.id,
            name: m.name,
            ip: m.ip,
            key: m.key,
            machines,
            current_machine_id: m.id,
            enable_apply: state.enable_apply,
            is_admin: true,
        },
        _ => {
            return Html("Machine not found".to_string());
        }
    };

    Html(render_template(template).unwrap_or_else(|e| e.to_string()))
}

async fn delete_machine_page(
    Path(mid): Path<i32>,
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let machine_result = QueryCore::find_machine_by_id(&state.conn, mid).await;
    let machines = QueryCore::fetch_machines_for_list(&state.conn).await.unwrap_or_default();

    let template = match machine_result {
        Ok(Some(m)) => DeleteTemplate {
            site_name: state.site_name.clone(),
            item_type: "机器".to_string(),
            name: m.name,
            ip: m.ip,
            domain: "".to_string(),
            ipv4: "".to_string(),
            ipv6: "".to_string(),
            machines,
            current_machine_id: m.id,
            enable_apply: state.enable_apply,
            is_admin: true,
        },
        _ => {
            return Html("Machine not found".to_string());
        }
    };

    Html(render_template(template).unwrap_or_else(|e| e.to_string()))
}
