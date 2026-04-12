use axum::Router;
use std::sync::Arc;

use crate::core::AppState;
use crate::handlers::{common, pages};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .merge(pages::index::create_router())
        .merge(pages::admin::create_router())
        .merge(pages::application::create_router())
        .merge(common::machines::create_router())
        .merge(common::targets::create_router())
}
