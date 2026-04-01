use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::{
    templates::{IndexTemplate, Machine, MachineTemplate, Target},
    AppState,
};
use server_service::query::Query;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index_page))
        .route("/m/{mid}", get(machine_page))
}

async fn index_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let targets: Vec<Target> = match Query::find_targets(&state.conn).await {
        Ok(list) => list
            .into_iter()
            .map(|t| Target {
                id: t.id,
                name: t.name,
                updated: 0,
            })
            .collect(),
        Err(_) => vec![],
    };

    let template = IndexTemplate { targets };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn machine_page(
    Path(mid): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let machine_result = Query::find_machine_by_id(&state.conn, mid).await;
    let machine = match machine_result {
        Ok(Some(m)) => Machine {
            id: m.id,
            name: m.name,
            ip: m.ip,
        },
        _ => {
            return Html("Machine not found".to_string());
        }
    };

    let targets: Vec<Target> = match Query::find_targets_by_machine_id(&state.conn, mid).await {
        Ok(list) => list
            .into_iter()
            .map(|t| Target {
                id: t.id,
                name: t.name,
                updated: 0,
            })
            .collect(),
        Err(_) => vec![],
    };

    let template = MachineTemplate { machine, targets };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}
