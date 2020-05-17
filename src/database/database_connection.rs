use dotenv::dotenv;
use std::env;
use mysql::{Pool, PooledConn};

pub fn start_db_connection() -> Pool{
    dotenv().ok();
    return Pool::new(env::var("DBURL").expect("DBURL MUST BE SET")).unwrap();
}

pub fn create_account(username: &str, password: &str, conn: PooledConn){
    // conn.query_first("");
}