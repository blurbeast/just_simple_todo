use diesel::{Connection, PgConnection};

pub fn run_db_connection(db_url: &str) -> PgConnection {
    PgConnection::establish(db_url).expect("could not connect to the database")
}