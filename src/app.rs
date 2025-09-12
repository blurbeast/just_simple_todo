use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::net::SocketAddr;
use crate::db::run_db_connection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// struct AppState {
//     pub db_pool: DbPool,
//     pub db_url: String,
//     pub port: u16,
//     pub listen_address: SocketAddr,
// }
// 
// impl AppState {
//     pub fn new() -> Self {
//         let port = set_env_var("PORT")
//             .parse::<u16>()
//             .expect("could not parse provided port");
//         let db_url = set_env_var("DATABASE_URL");
// 
//         let db_pool = run_db_connection(&*db_url);
// 
//         let addr = SocketAddr::from(([127, 0, 0, 1], port));
// 
//         AppState {
//             db_pool,
//             db_url,
//             port,
//             listen_address: addr,
//         }
//     }
// }

pub(crate) fn set_env_var(name: &str) -> String {
    std::env::var(name).expect("could not read from the environment")
}

