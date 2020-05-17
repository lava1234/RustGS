extern crate byte;
#[macro_use] extern crate enum_primitive;
use std::net::TcpListener;
pub mod enums;
mod client_handler;

const LOCAL_ADDRESS: &str = "127.0.0.1:5555";

fn main() {
    let server = TcpListener::bind(LOCAL_ADDRESS).expect("Could not bind");
    for stream in server.incoming() {
        match stream{
            Err(err) => {
                eprintln!("{}", err);
            }
            Ok(stream) => {
                std::thread::spawn(move || {
                    client_handler::handle_client(stream).unwrap_or_else(|err| eprintln!("{:?}", err))
                });
            }
        }
    }
}
