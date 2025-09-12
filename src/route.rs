// use crate::app::AppState;
use crate::auth::{login, register};
use axum::{routing::{post, get,}, Router};
use std::sync::Arc;
use crate::AppState;
use crate::handlers::{create_todo, delete_todo, find_todo_by_id, find_todos_by_user, update_todo};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        // Auth routes
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/todos", post(create_todo).get(find_todos_by_user))
        .route("/todos/{todo_id}", get(find_todo_by_id)
            .put(update_todo)
            .delete(delete_todo))
        .with_state(app_state)
}