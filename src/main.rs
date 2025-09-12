mod app;
mod auth;
mod db;
mod handlers;
mod models;
mod route;
mod schema;

use crate::app::AppState;
use axum::Router;
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_state = AppState::new();

    let listener = TcpListener::bind(&app_state.listen_address)
        .await
        .expect("could not listen to the specified port");
}
