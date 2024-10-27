use super::r#static::CHARSET_LEN;
use std::fmt;

#[derive(Debug, Clone)]
pub enum CryptError {
    CharsetError,
}

impl fmt::Display for CryptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptError::CharsetError => write!(f, "CryptError: Charset is invalid. Please ensure the charset contains exactly {} unique characters.",CHARSET_LEN ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DecryptError {
    CharsetError,
}

impl fmt::Display for DecryptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecryptError::CharsetError => write!(f, "CryptError: Charset is invalid. Please ensure the charset contains exactly {} unique characters.", CHARSET_LEN),
        }
    }
}
