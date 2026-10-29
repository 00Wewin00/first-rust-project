use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;
mod client_handler;
mod send_msg;

fn main() -> std::io::Result<()> {
    // 1. Биндим порт
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Сервер запущен на 127.0.0.1:8080");
    for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [0; 512];
            let n = stream.read(&mut buffer)?;
            if n == 0 {
            break; // Клиент отключился
        }
        println!("Получено байт: {}, текст: {}", n, String::from_utf8_lossy(&buffer[..n]));
        }
    Ok(())
    }
