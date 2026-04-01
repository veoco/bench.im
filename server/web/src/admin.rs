use askama::Template;
use axum::{
    response::Html,
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::{
    templates::{AdminIndexTemplate, AdminLoginTemplate},
    AppState,
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/login", get(admin_login_page))
        .route("/admin/", get(admin_index_page))
}

async fn admin_login_page() -> Html<String> {
    let template = AdminLoginTemplate;
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

async fn admin_index_page() -> Html<String> {
    let template = AdminIndexTemplate;
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}
