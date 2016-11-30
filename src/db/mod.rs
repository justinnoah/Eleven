use r2d2;
use r2d2_sqlite::SqliteConnectionManager;


pub fn init_db(db_uri: &str) -> r2d2::Pool<SqliteConnectionManager> {
    let config = r2d2::Config::default();
    let manager = SqliteConnectionManager::new(db_uri);
    let pool = r2d2::Pool::new(config, manager).unwrap();
    pool
}
