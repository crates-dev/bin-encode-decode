use crate::*;

#[derive(Debug, Clone)]
pub enum EncodeError {
    CharsetError,
}

impl fmt::Display for EncodeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodeError::CharsetError => write!(
                f,
                "EncodeError: Charset is invalid. Please ensure the charset contains exactly {} unique characters.",
                CHARSET_LEN
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DecodeError {
    CharsetError,
}

impl fmt::Display for DecodeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::CharsetError => write!(
                f,
                "DecodeError: Charset is invalid. Please ensure the charset contains exactly {} unique characters.",
                CHARSET_LEN
            ),
        }
    }
}
