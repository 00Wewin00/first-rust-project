use std::collections::HashMap;

#[derive(Debug)]
pub enum RecvStatus {
    Success(Vec<u8>),
    ClientDisconnected,
}
