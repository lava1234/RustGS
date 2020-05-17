use std::net::TcpStream;
use std::io::{Read, Write};
use crate::enums;
use crate::enum_primitive::*;
use byte::*;
use mysql::PooledConn;


const MAX_BUFFER_SIZE: usize = 32;

pub fn handle_client(mut stream: TcpStream, conn: PooledConn) -> core::result::Result<(), std::io::Error>{
    println!("Incoming conn");
    let mut buffer = [0; MAX_BUFFER_SIZE];
    loop {
        let buffer_size = stream.read(&mut buffer)?;
        let opcode: u16 = extract_opcode(&buffer);
        println!("{:?}", opcode);
        handle_command(opcode, &buffer[2..], &conn);
    }
}

fn extract_opcode(mut buffer: &[u8]) -> u16{
    return buffer[..2].read_with::<u16>(&mut 0, LE).unwrap();
}

fn handle_command(opcode: u16, buffer_data: &[u8], conn: &PooledConn){
    match enums::client_enums::ClientOpcode::from_u16(opcode){
        Some(enums::client_enums::ClientOpcode::PONG) => {
            println!("PONG");
        }
        Some(enums::client_enums::ClientOpcode::FIGHT_REQUEST) => {
            println!("FIGHT REQ");
        }
        Some(enums::client_enums::ClientOpcode::MOVE_REQUEST) => {
            println!("MOVE REQ");
        }
        Some(enums::client_enums::ClientOpcode::CREATE_ACCOUNT) => {
            create_account(buffer_data, &conn)
        }
        Some(enums::client_enums::ClientOpcode::CREATE_CHARACTER) => {
            println!("MOVE REQ");
        }
        None => {
            println!("Opcode not recognised")
        }
    }
}

fn create_account(buffer_data: &[u8], conn: &PooledConn){
    // TODO EXTRACT USERNAME AND PASS FROM BUFFER
}