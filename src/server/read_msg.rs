use std::net::TcpStream;
use std::io::{Write,Read};
use crate::types::RecvStatus;
pub fn read_msg(stream:&mut TcpStream)-> RecvStatus{
    let mut buffer_usize = [0u8;4];
    let client_byte = stream.read(&mut buffer_usize);


}