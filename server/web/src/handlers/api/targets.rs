use std::sync::Arc;

use axum::{
    extract::State,
    routing::get,
    Json, Router,
};

use server_service::output::{Target, TargetPublic};

use crate::core::{ApiClient, ApiError, AppState};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/targets/", get(list_targets))
        .route("/api/client/targets/", get(list_targets_client))
}

pub async fn list_targets(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TargetPublic>>, ApiError> {
    let targets = state.target_service().find_all().await?;
    Ok(Json(targets))
}

pub async fn list_targets_client(
    State(state): State<Arc<AppState>>,
    _: ApiClient,
) -> Result<Json<Vec<Target>>, ApiError> {
    // 客户端需要完整信息
    let targets = state.target_service().find_all().await?;
    Ok(Json(targets))
}
