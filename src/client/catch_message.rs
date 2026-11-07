use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::mpsc::Sender;
use super::read_msg::read_msg;
use super::types::RecvStatus::Success;
pub fn catch_message(stream:&mut TcpStream,disconnected_tx :& Sender<()>){
    let mut msg_byte :Vec<u8>= Vec::new();
    loop{
    match read_msg(stream){
        Success(n)=>{
            msg_byte = n;
        }
        (_)=>{
            println!("error in read_msg");
            disconnected_tx.send(());
            break;
        }
    }
    let msg_string = String::from_utf8_lossy(&msg_byte);
    println!("client sent: {}",msg_string);
}
    }