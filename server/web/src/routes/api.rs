use axum::Router;
use std::sync::Arc;

use crate::core::AppState;
use crate::handlers::api;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .merge(api::machines::create_router())
        .merge(api::targets::create_router())
        .merge(api::pings::create_router())
}
