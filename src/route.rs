use crate::app::AppState;
use axum::Router;
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        // .route("/", get())
        .with_state(app_state)
}