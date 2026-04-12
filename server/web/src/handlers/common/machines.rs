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

use server_service::input::{CreateMachineRequest, UpdateMachineRequest};
use server_service::output::Machine;

use crate::core::{AdminAuth, AdminUserWeb, ApiError, AppState, render_template};
use crate::templates::pages::{DeleteTemplate, EditMachineTemplate};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
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

pub async fn create_machine_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Valid(Json(req)): Valid<Json<CreateMachineRequest>>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let machine = state.machine_service().create(req).await?;

    Ok((StatusCode::CREATED, Json(json!({"msg": machine.id}))))
}

pub async fn edit_machine_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(mid): Path<i32>,
    Valid(Json(req)): Valid<Json<UpdateMachineRequest>>,
) -> Result<Json<Value>, ApiError> {
    state.machine_service().update(mid, req).await?;
    Ok(Json(json!({"msg": "success"})))
}

pub async fn list_machines_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
) -> Result<Json<Vec<Machine>>, ApiError> {
    let machines = state.machine_service().find_all().await?;
    Ok(Json(machines))
}

pub async fn get_machine_by_mid_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(mid): Path<i32>,
) -> Result<Json<Machine>, ApiError> {
    let machine: Machine = state
        .machine_service()
        .find_by_id(mid)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Machine {} not found", mid)))?;

    Ok(Json(machine))
}

pub async fn delete_machine_by_mid_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(mid): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    state.machine_service().delete(mid).await?;
    Ok(Json(json!({"msg": "success"})))
}

// 页面 handlers
async fn new_machine_page(
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let template = EditMachineTemplate {
        site_name: state.site_name().to_string(),
        is_edit: false,
        id: 0,
        name: "".to_string(),
        ip: "".to_string(),
        key: "".to_string(),
        machines: state.get_sidebar_machines().await,
        current_machine_id: 0,
        enable_apply: state.enable_apply(),
        is_admin: true,
    };
    Html(render_template(template).unwrap_or_else(|_| "<h1>页面加载失败</h1><p>请稍后重试</p>".to_string()))
}

async fn edit_machine_page(
    Path(mid): Path<i32>,
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let template = match state.machine_service().find_by_id::<Machine>(mid).await {
        Ok(Some(m)) => EditMachineTemplate {
            site_name: state.site_name().to_string(),
            is_edit: true,
            id: m.id,
            name: m.name.clone(),
            ip: m.ip.clone(),
            key: m.key.clone(),
            machines: state.get_sidebar_machines().await,
            current_machine_id: m.id,
            enable_apply: state.enable_apply(),
            is_admin: true,
        },
        _ => {
            return Html("Machine not found".to_string());
        }
    };

    Html(render_template(template).unwrap_or_else(|_| "<h1>页面加载失败</h1><p>请稍后重试</p>".to_string()))
}

async fn delete_machine_page(
    Path(mid): Path<i32>,
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let template = match state.machine_service().find_by_id::<Machine>(mid).await {
        Ok(Some(m)) => DeleteTemplate::for_machine(
            state.site_name().to_string(),
            m.name,
            m.ip,
            state.get_sidebar_machines().await,
            m.id,
            state.enable_apply(),
        ),
        _ => {
            return Html("Machine not found".to_string());
        }
    };

    Html(render_template(template).unwrap_or_else(|_| "<h1>页面加载失败</h1><p>请稍后重试</p>".to_string()))
}
