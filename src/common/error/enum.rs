#[derive(Debug, Clone)]
pub enum EncodeError {
    CharsetError,
}

#[derive(Debug, Clone)]
pub enum DecodeError {
    CharsetError,
}
