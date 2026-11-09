use std::net::TcpListener;
use std::thread;
mod mmsg;
mod msgall;
mod types;
mod clients;
mod client_handler;
use crate::client_handler::clienthandler;
fn main() -> std::io::Result<()> {
    // 1. Биндим порт
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Сервер запущен на 127.0.0.1:8080");
    for stream in listener.incoming() {
        let stream =stream?;
        thread::spawn(move || {
            if let Err(error) = clienthandler(stream){
                eprintln!("error client : {error}");
            }
        });
        }
    Ok(())
}