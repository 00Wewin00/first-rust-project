#[derive(Debug)]
pub enum RecvStatus {
    Success(Vec<u8>),
    Timeout,
    CorruptedData,
    ClientDisconnected,
}