use std::net::TcpStream;
use std::io::{Read, Write};
mod send_msg;
fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8008")?;
    let message = b"hello , I am client from rust";
    stream.write_all(message)?;
    // 3. Читаем ответ от сервера
    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;

    println!("Ответ от сервера: {}", String::from_utf8_lossy(&buffer[..n]));
    
    Ok(())
}

