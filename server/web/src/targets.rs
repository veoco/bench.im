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

use server_service::input::{CreateTargetRequest, UpdateTargetRequest};
use server_service::output::TargetResponse;

use crate::extractors::{AdminAuth, AdminUserWeb, ApiClient};
use crate::{
    templates::{DeleteTemplate, EditTargetTemplate},
    ApiError, AppState, render_template,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        // API 路由
        .route("/api/targets/", get(list_targets))
        .route("/api/client/targets/", get(list_targets_client))
        .route(
            "/api/admin/targets/",
            axum::routing::post(create_target_admin).get(list_targets_admin),
        )
        .route(
            "/api/admin/targets/{tid}",
            get(get_target_by_tid_admin)
                .post(edit_target_admin)
                .delete(delete_target_admin),
        )
        // 页面路由
        .route("/admin/targets/new", get(new_target_page))
        .route("/admin/targets/{tid}", get(edit_target_page))
        .route("/admin/targets/{tid}/delete", get(delete_target_page))
}

pub async fn list_targets(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TargetResponse>>, ApiError> {
    let targets = state.target_service().find_all().await?;
    Ok(Json(targets))
}

pub async fn list_targets_client(
    State(state): State<Arc<AppState>>,
    _: ApiClient,
) -> Result<Json<Vec<entity::target::Model>>, ApiError> {
    // 客户端需要完整信息
    let targets = state.target_service().find_all_admin().await?;
    Ok(Json(targets))
}

pub async fn create_target_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Valid(Json(req)): Valid<Json<CreateTargetRequest>>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    // 检查是否已存在
    if state
        .target_service()
        .find_by_name(&req.name)
        .await?
        .is_some()
    {
        return Err(ApiError::Conflict(
            "Target with this name already exists".to_string(),
        ));
    }

    state.target_service().create(req).await?;
    Ok((StatusCode::CREATED, Json(json!({"msg": "success"}))))
}

pub async fn edit_target_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(tid): Path<i32>,
    Valid(Json(req)): Valid<Json<UpdateTargetRequest>>,
) -> Result<Json<Value>, ApiError> {
    state.target_service().ensure_exists(tid).await?;

    state.target_service().update(tid, req).await?;
    Ok(Json(json!({"msg": "success"})))
}

pub async fn list_targets_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
) -> Result<Json<Vec<entity::target::Model>>, ApiError> {
    let targets = state.target_service().find_all_admin().await?;
    Ok(Json(targets))
}

pub async fn get_target_by_tid_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(tid): Path<i32>,
) -> Result<Json<entity::target::Model>, ApiError> {
    let target = state
        .target_service()
        .find_by_id_admin(tid)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Target {} not found", tid)))?;

    Ok(Json(target))
}

pub async fn delete_target_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(tid): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    // 检查 target 存在
    state.target_service().ensure_exists(tid).await?;

    state.target_service().delete(tid).await?;
    Ok(Json(json!({"msg": "success"})))
}

// 页面 handlers
async fn new_target_page(
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let template = EditTargetTemplate {
        site_name: state.site_name().to_string(),
        is_edit: false,
        id: 0,
        name: "".to_string(),
        domain: "".to_string(),
        ipv4: "".to_string(),
        ipv6: "".to_string(),
        machines: state.get_sidebar_machines().await,
        current_machine_id: 0,
        enable_apply: state.enable_apply(),
        is_admin: true,
    };
    Html(render_template(template).unwrap_or_else(|e| e.to_string()))
}

async fn edit_target_page(
    Path(tid): Path<i32>,
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let template = match state.target_service().find_by_id_admin(tid).await {
        Ok(Some(t)) => EditTargetTemplate {
            site_name: state.site_name().to_string(),
            is_edit: true,
            id: t.id,
            name: t.name,
            domain: t.domain.unwrap_or_default(),
            ipv4: t.ipv4.unwrap_or_default(),
            ipv6: t.ipv6.unwrap_or_default(),
            machines: state.get_sidebar_machines().await,
            current_machine_id: 0,
            enable_apply: state.enable_apply(),
            is_admin: true,
        },
        _ => {
            return Html("Target not found".to_string());
        }
    };

    Html(render_template(template).unwrap_or_else(|e| e.to_string()))
}

async fn delete_target_page(
    Path(tid): Path<i32>,
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let template = match state.target_service().find_by_id_admin(tid).await {
        Ok(Some(t)) => DeleteTemplate {
            site_name: state.site_name().to_string(),
            item_type: "目标".to_string(),
            name: t.name,
            ip: "".to_string(),
            domain: t.domain.unwrap_or_default(),
            ipv4: t.ipv4.unwrap_or_default(),
            ipv6: t.ipv6.unwrap_or_default(),
            machines: state.get_sidebar_machines().await,
            current_machine_id: 0,
            enable_apply: state.enable_apply(),
            is_admin: true,
        },
        _ => {
            return Html("Target not found".to_string());
        }
    };

    Html(render_template(template).unwrap_or_else(|e| e.to_string()))
}
