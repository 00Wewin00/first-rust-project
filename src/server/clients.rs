use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
};
pub struct ClientState {
    pub ids: HashMap<u32, bool>,
    pub streams: HashMap<u32, TcpStream>,
}
pub type  SharedClientState = Arc<Mutex<ClientState>>;