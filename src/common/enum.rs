/// Represents errors that can occur during encoding.
///
/// These errors are related to character set validation and encoding process.
#[derive(Debug, Clone)]
pub enum EncodeError {
    /// Indicates invalid character set configuration.
    CharsetError,
}

/// Represents errors that can occur during decoding.
///
/// These errors are related to character set validation and decoding process.
#[derive(Debug, Clone)]
pub enum DecodeError {
    /// Indicates invalid character set configuration.
    CharsetError,
}
