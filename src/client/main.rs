use std::net::TcpStream;
use std::io::{Read, Write};
use std::io;
mod catch_message;
mod read_msg;
mod types;
use std::thread;
mod send_msg;
use crate::send_msg::clienthandler;
fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let stream_read =stream.try_clone()?;
    let (disconnected_tx, disconnected_rx) =std::sync::mpsc::channel::<()>();
    let reader_handle = thread::spawn(move || {
        let mut stream_read=stream_read;
        catch_message::catch_message(&mut stream_read, &disconnected_tx);
    });
    loop{
        match disconnected_rx.try_recv() {
            Ok(()) => {println!("Поток приёма сообщил об отключении");
            break;
        },
        Err(error) => eprintln!("Не удалось получить сигнал: {error}"),
    }
    let mut message = String::new();
    io::stdin().read_line(&mut message)?;
    message = message.trim_end().to_string();
    send_msg::clienthandler(&mut stream, &mut message);
    }
    if reader_handle.join().is_err() {
        eprintln!("Поток приёма завершился с паникой");
    }
    Ok(())
}

