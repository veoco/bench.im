use askama::Template;
use axum::{
    extract::State,
    response::Html,
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::{
    templates::{AdminIndexTemplate, AdminLoginTemplate, MachineForList},
    AppState,
};
use server_service::query::Query;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/login", get(admin_login_page))
        .route("/admin/", get(admin_index_page))
}

async fn fetch_machines_for_list(state: &Arc<AppState>) -> Vec<MachineForList> {
    match Query::find_machines(&state.conn).await {
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

async fn admin_login_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let machines = fetch_machines_for_list(&state).await;
    let template = AdminLoginTemplate { machines };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn admin_index_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let machines = fetch_machines_for_list(&state).await;
    let template = AdminIndexTemplate { machines };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}
