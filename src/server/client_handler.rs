use std::net::TcpStream;
use std::io::{Read, Write};
//use crate::types::RecvStatus::Success;

use super::read_msg;
use super::types;
pub fn clienthandler(mut socket :TcpStream) -> std::io::Result<()> {
    loop{
        match read_msg::read_msg(&mut socket){
            types::RecvStatus::Success(data)=>{
                let message_string :String=String::from_utf8_lossy(&data).into_owned();
                println!("{}",message_string);
            }
            _ =>{
                drop(socket);
                println!("client disconnected");
                break;
            }
        };

    }
    Ok(())
}