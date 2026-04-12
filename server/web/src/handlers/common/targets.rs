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
use server_service::output::Target;

use crate::core::{AdminAuth, AdminUserWeb, ApiError, AppState, render_template};
use crate::templates::pages::{DeleteTemplate, EditTargetTemplate};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
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

pub async fn create_target_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Valid(Json(req)): Valid<Json<CreateTargetRequest>>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    // 检查是否已存在
    if state
        .target_service()
        .find_by_name::<Target>(&req.name)
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
    state.target_service().update(tid, req).await?;
    Ok(Json(json!({"msg": "success"})))
}

pub async fn list_targets_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
) -> Result<Json<Vec<Target>>, ApiError> {
    let targets = state.target_service().find_all().await?;
    Ok(Json(targets))
}

pub async fn get_target_by_tid_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(tid): Path<i32>,
) -> Result<Json<Target>, ApiError> {
    let target = state
        .target_service()
        .find_by_id(tid)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Target {} not found", tid)))?;

    Ok(Json(target))
}

pub async fn delete_target_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(tid): Path<i32>,
) -> Result<Json<Value>, ApiError> {
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
    let template = match state.target_service().find_by_id::<Target>(tid).await {
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
    let template = match state.target_service().find_by_id::<Target>(tid).await {
        Ok(Some(t)) => DeleteTemplate::for_target(
            state.site_name().to_string(),
            t.name,
            t.domain,
            t.ipv4,
            t.ipv6,
            state.get_sidebar_machines().await,
            state.enable_apply(),
        ),
        _ => {
            return Html("Target not found".to_string());
        }
    };

    Html(render_template(template).unwrap_or_else(|e| e.to_string()))
}
