mod models;
mod schema;
mod handlers;
mod auth;
mod db;
mod app;
mod route;

use std::env;
use std::net::SocketAddr;
use axum::Router;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use crate::app::AppState;

#[tokio::main]
async fn main() {

    dotenv().ok();


    let app_state = AppState::new();

    let listener = TcpListener::bind(&app_state.listen_address)
        .await
        .expect("could not listen to the specified port");

}
