mod app;
mod auth;
mod db;
mod handlers;
mod models;
mod route;
mod schema;

use std::net::SocketAddr;
use std::sync::Arc;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use r2d2::Pool;
use tokio::net::TcpListener;
use crate::app::set_env_var;
use crate::route::create_router;


pub struct AppState {
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
    pub listen_address: SocketAddr,
}
#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    let port = set_env_var("PORT")
        .parse::<u16>()
        .expect("could not parse provided port");
    let db_url = set_env_var("DATABASE_URL");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let db_pool = diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pool");

    // Get listen address from environment
    let listen_address = std::env::var("LISTEN_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    // Create AppState
    let state = Arc::new(AppState {
        db_pool,
        listen_address: listen_address.parse().expect("Invalid listen address"),
    });

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