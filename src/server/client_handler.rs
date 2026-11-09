use std::net::TcpStream;
use super::mmsg;
use super::types;
pub fn clienthandler(mut socket :TcpStream) -> std::io::Result<()> {
    loop{
        match mmsg::read_msg(&mut socket){
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