use super::*;

impl fmt::Display for EncodeError {
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

impl fmt::Display for DecodeError {
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
