use std::sync::Arc;

use askama::Template;
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
    templates::{DeleteTemplate, EditTargetTemplate, MachineForList},
    AppState,
};
use server_service::{
    Mutation as MutationCore, Query as QueryCore, TargetCreateAdmin, TargetPublic,
};

async fn fetch_machines_for_list(state: &Arc<AppState>) -> Vec<MachineForList> {
    match QueryCore::find_machines(&state.conn).await {
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

pub async fn list_targets(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(targets) = QueryCore::find_targets(&state.conn).await {
        let mut outputs = vec![];
        for t in targets {
            outputs.push(TargetPublic::from(t));
        }
        res = json!(outputs);
        status = StatusCode::OK;
    }

    (status, Json(res))
}

pub async fn list_targets_client(
    State(state): State<Arc<AppState>>,
    _: ApiClient,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(targets) = QueryCore::find_targets(&state.conn).await {
        res = json!(targets);
        status = StatusCode::OK;
    }

    (status, Json(res))
}

pub async fn create_target_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Valid(Json(target_create)): Valid<Json<TargetCreateAdmin>>,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(Some(_)) = QueryCore::find_target_by_name(&state.conn, &target_create.name).await {
        status = StatusCode::CONFLICT;
        res = json!({"msg": "already exists"});
    } else {
        match MutationCore::create_target(&state.conn, &target_create).await {
            Ok(_) => {
                status = StatusCode::CREATED;
                res = json!({"msg": "success"});
            }
            Err(_) => {}
        }
    }

    (status, Json(res))
}

pub async fn edit_target_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(tid): Path<i32>,
    Valid(Json(target_create)): Valid<Json<TargetCreateAdmin>>,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(Some(target)) = QueryCore::find_target_by_id(&state.conn, tid).await {
        let _ = MutationCore::edit_target(&state.conn, target.id, &target_create).await;
        res = json!({"msg": "success"});
        status = StatusCode::OK;
    }

    (status, Json(res))
}

pub async fn list_targets_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(targets) = QueryCore::find_targets(&state.conn).await {
        res = json!(targets);
        status = StatusCode::OK;
    }

    (status, Json(res))
}

pub async fn get_target_by_tid_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(tid): Path<i32>,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(Some(target)) = QueryCore::find_target_by_id(&state.conn, tid).await {
        res = json!(target);
        status = StatusCode::OK;
    }

    (status, Json(res))
}

pub async fn delete_target_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(tid): Path<i32>,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(Some(_)) = QueryCore::find_target_by_id(&state.conn, tid).await {
        let _ = MutationCore::delete_target(&state.conn, tid).await;
        res = json!({"msg": "success"});
        status = StatusCode::OK;
    }

    (status, Json(res))
}

// 页面 handlers
async fn new_target_page(
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let machines = fetch_machines_for_list(&state).await;
    let template = EditTargetTemplate {
        site_name: state.site_name.clone(),
        is_edit: false,
        id: 0,
        name: "".to_string(),
        domain: "".to_string(),
        ipv4: "".to_string(),
        ipv6: "".to_string(),
        machines,
        current_machine_id: 0,
        enable_apply: state.enable_apply,
        is_admin: true,
    };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn edit_target_page(
    Path(tid): Path<i32>,
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let target_result = QueryCore::find_target_by_id(&state.conn, tid).await;
    let machines = fetch_machines_for_list(&state).await;
    let template = match target_result {
        Ok(Some(t)) => EditTargetTemplate {
            site_name: state.site_name.clone(),
            is_edit: true,
            id: t.id,
            name: t.name,
            domain: t.domain.unwrap_or_default(),
            ipv4: t.ipv4.unwrap_or_default(),
            ipv6: t.ipv6.unwrap_or_default(),
            machines,
            current_machine_id: 0,
            enable_apply: state.enable_apply,
            is_admin: true,
        },
        _ => {
            return Html("Target not found".to_string());
        }
    };

    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn delete_target_page(
    Path(tid): Path<i32>,
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let target_result = QueryCore::find_target_by_id(&state.conn, tid).await;
    let machines = fetch_machines_for_list(&state).await;
    let template = match target_result {
        Ok(Some(t)) => DeleteTemplate {
            site_name: state.site_name.clone(),
            item_type: "目标".to_string(),
            name: t.name,
            ip: "".to_string(),
            domain: t.domain.unwrap_or_default(),
            ipv4: t.ipv4.unwrap_or_default(),
            ipv6: t.ipv6.unwrap_or_default(),
            machines,
            current_machine_id: 0,
            enable_apply: state.enable_apply,
            is_admin: true,
        },
        _ => {
            return Html("Target not found".to_string());
        }
    };

    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}
