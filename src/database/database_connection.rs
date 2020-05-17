use dotenv::dotenv;
use std::env;
use mysql::*;
use mysql::prelude::*;

pub fn start_db_connection() -> Pool{
    dotenv().ok();
    return Pool::new(env::var("DBURL").expect("DBURL MUST BE SET")).unwrap();
}

pub fn create_account(username: &str, password: &str, conn: &mut PooledConn){
    match conn.exec_first::<i32, _, _>("INSERT INTO users(username, password) VALUES (?, ?)", (username, password)).map(Option::unwrap) {
        Ok(_) => println!("User added"),
        Err(err) => println!("{:?}", err)
    };
}