mod models;
mod schema;
mod handlers;
mod auth;

use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let db_url = env::var("DATBASE_URL").expect("could not load environments");

    let conn = PgConnection::establish(&db_url).expect("could not connect to the db");

    // println!("Hello, world!");
}
