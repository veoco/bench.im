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

use crate::extractors::{AdminAuth, AdminUserWeb, ApiClient};
use crate::{
    templates::{DeleteTemplate, EditTargetTemplate},
    ApiError, AppState, render_template,
};
use server_service::{
    Mutation as MutationCore, Query as QueryCore, TargetCreateAdmin, TargetPublic,
};
use entity::target::Model as Target;

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

pub async fn list_targets(State(state): State<Arc<AppState>>) -> Result<Json<Vec<TargetPublic>>, ApiError> {
    let targets = QueryCore::find_targets(&state.db()).await?;
    Ok(Json(targets.into_iter().map(TargetPublic::from).collect()))
}

pub async fn list_targets_client(
    State(state): State<Arc<AppState>>,
    _: ApiClient,
) -> Result<Json<Vec<Target>>, ApiError> {
    let targets = QueryCore::find_targets(&state.db()).await?;
    Ok(Json(targets))
}

pub async fn create_target_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Valid(Json(target_create)): Valid<Json<TargetCreateAdmin>>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    // 检查是否已存在
    if QueryCore::find_target_by_name(&state.db(), &target_create.name).await?.is_some() {
        return Err(ApiError::Conflict("Target with this name already exists".to_string()));
    }

    MutationCore::create_target(&state.db(), &target_create).await?;
    Ok((StatusCode::CREATED, Json(json!({"msg": "success"}))))
}

pub async fn edit_target_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(tid): Path<i32>,
    Valid(Json(target_create)): Valid<Json<TargetCreateAdmin>>,
) -> Result<Json<Value>, ApiError> {
    let target = QueryCore::find_target_by_id(&state.db(), tid)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Target {} not found", tid)))?;

    MutationCore::edit_target(&state.db(), target.id, &target_create).await?;
    Ok(Json(json!({"msg": "success"})))
}

pub async fn list_targets_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
) -> Result<Json<Vec<Target>>, ApiError> {
    let targets = QueryCore::find_targets(&state.db()).await?;
    Ok(Json(targets))
}

pub async fn get_target_by_tid_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(tid): Path<i32>,
) -> Result<Json<Target>, ApiError> {
    let target = QueryCore::find_target_by_id(&state.db(), tid)
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
    let _ = QueryCore::find_target_by_id(&state.db(), tid)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Target {} not found", tid)))?;

    MutationCore::delete_target(&state.db(), tid).await?;
    Ok(Json(json!({"msg": "success"})))
}

// 页面 handlers
async fn new_target_page(
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let machines = QueryCore::fetch_machines_for_list(&state.db()).await.unwrap_or_default();
    let template = EditTargetTemplate {
        site_name: state.site_name().to_string(),
        is_edit: false,
        id: 0,
        name: "".to_string(),
        domain: "".to_string(),
        ipv4: "".to_string(),
        ipv6: "".to_string(),
        machines,
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
    let target_result = QueryCore::find_target_by_id(&state.db(), tid).await;
    let machines = QueryCore::fetch_machines_for_list(&state.db()).await.unwrap_or_default();

    let template = match target_result {
        Ok(Some(t)) => EditTargetTemplate {
            site_name: state.site_name().to_string(),
            is_edit: true,
            id: t.id,
            name: t.name,
            domain: t.domain.unwrap_or_default(),
            ipv4: t.ipv4.unwrap_or_default(),
            ipv6: t.ipv6.unwrap_or_default(),
            machines,
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
    let target_result = QueryCore::find_target_by_id(&state.db(), tid).await;
    let machines = QueryCore::fetch_machines_for_list(&state.db()).await.unwrap_or_default();

    let template = match target_result {
        Ok(Some(t)) => DeleteTemplate {
            site_name: state.site_name().to_string(),
            item_type: "目标".to_string(),
            name: t.name,
            ip: "".to_string(),
            domain: t.domain.unwrap_or_default(),
            ipv4: t.ipv4.unwrap_or_default(),
            ipv6: t.ipv6.unwrap_or_default(),
            machines,
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
