extern crate byte;
#[macro_use] extern crate enum_primitive;
use std::net::TcpListener;
pub mod enums;
pub mod database;
mod client_handler;
use mysql::Pool;

const LOCAL_ADDRESS: &str = "127.0.0.1:5555";

fn main() {
    let server = TcpListener::bind(LOCAL_ADDRESS).expect("Could not bind");
    let conn_pool = database::database_connection::start_db_connection();
    for stream in server.incoming() {
        match stream{
            Err(err) => {
                eprintln!("{}", err);
            }
            Ok(stream) => {
                let pool_clone = conn_pool.clone();
                std::thread::spawn(move || {
                    let mut conn = pool_clone.get_conn().unwrap();
                    client_handler::handle_client(stream, &mut conn).unwrap_or_else(|err| eprintln!("{:?}", err))
                });
            }
        }
    }
}
