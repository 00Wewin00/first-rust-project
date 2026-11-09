use std::net::TcpStream;
use std::io::{Read,Write};
use crate::types::{self, RecvStatus};
pub fn read_msg(stream:&mut TcpStream)-> RecvStatus{
    let mut buffer_size_bytes = [0u8;4];
    let buffer_size_usize:usize=match stream.read_exact(&mut buffer_size_bytes){
        Ok(())=>{
            
            let buffer_size_u32 = u32::from_be_bytes(buffer_size_bytes);
            buffer_size_u32 as usize
        }
        Err(_) => {
            return types::RecvStatus::ClientDisconnected;
        }
            };
        let mut buffer_message_byte=vec![0u8;buffer_size_usize];
        match stream.read_exact(&mut buffer_message_byte){
            Ok(())=>{
                return types::RecvStatus::Success(buffer_message_byte);

            }
            Err(_)=>{
                return types::RecvStatus::ClientDisconnected;
            }
        }
}
pub fn send_message(stream:& mut TcpStream,message :&mut String) -> std::io::Result<()> {
    let message_lenght_usize = message.len();
    let message_lenght_size = message_lenght_usize as u32;
    let size_bytes =message_lenght_size.to_be_bytes();
    stream.write_all(&size_bytes)?;
    stream.write_all(message.as_bytes())?;
    Ok(())
}
