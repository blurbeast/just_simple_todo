mod app;
mod auth;
mod db;
mod handlers;
mod models;
mod route;
mod schema;

use crate::route::create_router;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenvy::dotenv;
use r2d2::Pool;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;


pub struct AppState {
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
    pub listen_address: SocketAddr,
}
#[tokio::main]
async fn main() {

    dotenv().ok();

    let state = Arc::new(AppState::new());

    // Build router
    let app = create_router(state.clone());

    // Create TCP listener
    let listener = TcpListener::bind(&state.listen_address)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}