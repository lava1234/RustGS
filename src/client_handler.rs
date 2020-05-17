use std::net::TcpStream;
use std::io::{Read, Write};
use crate::enums;
use crate::enum_primitive::*;
use byte::*;
use mysql::PooledConn;
use crate::database;
use crate::logic;
// use database;

const MAX_BUFFER_SIZE: usize = 512;

pub fn handle_client(mut stream: TcpStream, conn: &mut PooledConn) -> core::result::Result<(), std::io::Error>{
    println!("Incoming conn");
    let mut buffer = [0; MAX_BUFFER_SIZE];
    loop {
        let buffer_size = stream.read(&mut buffer)?;
        let opcode: u16 = extract_opcode(&buffer);
        handle_command(opcode, &buffer[4..], conn);
    }
}

fn extract_opcode(mut buffer: &[u8]) -> u16{
    return buffer[..2].read_with::<u16>(&mut 0, LE).unwrap();
}

fn handle_command(opcode: u16, buffer_data: &[u8], mut conn: &mut PooledConn){
    match enums::client_enums::ClientOpcode::from_u16(opcode){
        Some(enums::client_enums::ClientOpcode::PONG) => {
            println!("PONG");
        }
        Some(enums::client_enums::ClientOpcode::FIGHT_REQUEST) => {
            logic::fight::parse_fighters(buffer_data);
        }
        Some(enums::client_enums::ClientOpcode::MOVE_REQUEST) => {
            println!("MOVE REQ");
        }
        Some(enums::client_enums::ClientOpcode::CREATE_ACCOUNT) => {
            create_account(buffer_data, &mut conn)
        }
        Some(enums::client_enums::ClientOpcode::CREATE_CHARACTER) => {
            println!("MOVE REQ");
        }
        None => {
            println!("Opcode not recognised")
        }
    }
}

fn create_account(buffer_data: &[u8], conn: &mut PooledConn){
    println!("creating acc");
    let name_size: usize = usize::from(buffer_data[0]) + 1;
    let username = std::str::from_utf8(&buffer_data[1..name_size]).unwrap();
    let pass_size: usize = usize::from(buffer_data[name_size]);
    let password = std::str::from_utf8(&buffer_data[name_size+1..pass_size+name_size+1]).unwrap();

    database::database_connection::create_account(username, password,  conn);
}