use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use server_service::output::{MachineDetail, MaskedMachine};

use crate::core::{ApiError, AppState};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/machines/", get(list_machines))
        .route("/api/machines/{mid}", get(get_machine_by_mid))
}

pub async fn list_machines(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<MaskedMachine>>, ApiError> {
    let machines = state.machine_service().find_all().await?;
    Ok(Json(machines))
}

pub async fn get_machine_by_mid(
    State(state): State<Arc<AppState>>,
    Path(mid): Path<i32>,
) -> Result<Json<MachineDetail>, ApiError> {
    use server_service::output::Machine;
    
    let machine: Machine = state
        .machine_service()
        .find_by_id(mid)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Machine {} not found", mid)))?;

    let targets = state.target_service().find_all().await?;
    Ok(Json(MachineDetail::new(&machine, targets)))
}
