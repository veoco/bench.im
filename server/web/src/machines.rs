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

use crate::extractors::{AdminAuth, AdminUserWeb};
use crate::{
    templates::{DeleteTemplate, EditMachineTemplate, MachineForList},
    AppState,
};
use server_service::{
    MachineCreateAdmin, MachinePublic, MachineTargetsPublic, Mutation as MutationCore,
    Query as QueryCore, TargetPublic,
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

pub async fn list_machines(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(machines) = QueryCore::find_machines(&state.conn).await {
        let mut outputs = vec![];
        for m in machines {
            outputs.push(MachinePublic::from(m));
        }
        res = json!(outputs);
        status = StatusCode::OK;
    }
    (status, Json(res))
}

pub async fn get_machine_by_mid(
    State(state): State<Arc<AppState>>,
    Path(mid): Path<i32>,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(Some(machine)) = QueryCore::find_machine_by_id(&state.conn, mid).await {
        let m = MachinePublic::from(machine);
        let mut m = MachineTargetsPublic::from(m);
        if let Ok(targets) = QueryCore::find_targets(&state.conn).await {
            let mut outputs = vec![];
            for t in targets {
                outputs.push(TargetPublic::from(t));
            }
            m.targets = outputs;
        }
        res = json!(m);
        status = StatusCode::OK;
    }

    (status, Json(res))
}

pub async fn create_machine_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Valid(Json(machine_create)): Valid<Json<MachineCreateAdmin>>,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(Some(_)) = QueryCore::find_machine_by_name(&state.conn, &machine_create.name).await {
        status = StatusCode::CONFLICT;
        res = json!({"msg": "already exists"});
    } else {
        match MutationCore::create_machine(&state.conn, &machine_create).await {
            Ok(m) => {
                let mid = m.id.as_ref();
                status = StatusCode::CREATED;
                res = json!({"msg": mid});
            }
            Err(_) => {}
        }
    }

    (status, Json(res))
}

pub async fn edit_machine_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(mid): Path<i32>,
    Valid(Json(machine_create)): Valid<Json<MachineCreateAdmin>>,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(Some(machine)) = QueryCore::find_machine_by_id(&state.conn, mid).await {
        let _ = MutationCore::edit_machine(&state.conn, machine.id, &machine_create).await;
        res = json!({"msg": "success"});
        status = StatusCode::OK;
    }

    (status, Json(res))
}

pub async fn list_machines_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(machines) = QueryCore::find_machines(&state.conn).await {
        res = json!(machines);
        status = StatusCode::OK;
    }
    (status, Json(res))
}

pub async fn get_machine_by_mid_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(mid): Path<i32>,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(Some(machine)) = QueryCore::find_machine_by_id(&state.conn, mid).await {
        res = json!(machine);
        status = StatusCode::OK;
    }

    (status, Json(res))
}

pub async fn delete_machine_by_mid_admin(
    State(state): State<Arc<AppState>>,
    _: AdminAuth,
    Path(mid): Path<i32>,
) -> (StatusCode, Json<Value>) {
    let mut res = json!({"msg": "failed"});
    let mut status = StatusCode::INTERNAL_SERVER_ERROR;

    if let Ok(Some(machine)) = QueryCore::find_machine_by_id(&state.conn, mid).await {
        let _ = MutationCore::delete_machine(&state.conn, machine.id).await;
        res = json!({"msg": "success"});
        status = StatusCode::OK;
    }

    (status, Json(res))
}

// 页面 handlers
async fn new_machine_page(
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let machines = fetch_machines_for_list(&state).await;
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
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn edit_machine_page(
    Path(mid): Path<i32>,
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let machine_result = QueryCore::find_machine_by_id(&state.conn, mid).await;
    let machines = fetch_machines_for_list(&state).await;
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

    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn delete_machine_page(
    Path(mid): Path<i32>,
    State(state): State<Arc<AppState>>,
    _: AdminUserWeb,
) -> Html<String> {
    let machine_result = QueryCore::find_machine_by_id(&state.conn, mid).await;
    let machines = fetch_machines_for_list(&state).await;
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

    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}
