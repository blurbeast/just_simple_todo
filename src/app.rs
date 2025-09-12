use crate::db::run_db_connection;
use crate::AppState;

impl AppState {
    pub fn new() -> Self {

        let db_url = set_env_var("DATABASE_URL");

        let db_pool = run_db_connection(&db_url);

        let listen_address = std::env::var("LISTEN_ADDRESS")
            .unwrap_or_else(|_| "0.0.0.0:3000".to_string());

        AppState {
            db_pool,
            listen_address: listen_address.parse().expect("Invalid listen address")
        }
    }
}

pub(crate) fn set_env_var(name: &str) -> String {
    std::env::var(name).expect("could not read from the environment")
}

