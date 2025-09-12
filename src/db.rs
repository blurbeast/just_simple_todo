use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

pub fn run_db_connection(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let db_pool = Pool::builder()
        .build(manager)
        .expect("failed to create pool");
    db_pool
}
