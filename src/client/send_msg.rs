use std::net::TcpStream;
use std::io::{Write};
pub fn clienthandler(stream:& mut TcpStream,message :&mut String) -> std::io::Result<()> {
    let message_lenght_usize = message.len();
    let message_lenght_size = message_lenght_usize as u32;
    let size_bytes =message_lenght_size.to_be_bytes();
    stream.write_all(&size_bytes)?;
    stream.write_all(message.as_bytes())?;
    Ok(())
}