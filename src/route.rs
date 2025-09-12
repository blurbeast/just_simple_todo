use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::app::AppState;

// pub fn create_router(app_state: Arc<AppState>) -> Router {
//
//     // Router::new()
//     //     .route("/", get())
//     //     .with_state(app_state)
// }