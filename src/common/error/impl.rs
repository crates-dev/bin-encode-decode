use super::*;

/// Provides display formatting for EncodeError.
///
/// Implements human-readable error messages for encoding failures.
impl fmt::Display for EncodeError {
    /// Formats the EncodeError for display purposes.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The formatter to use.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
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

/// Provides display formatting for DecodeError.
///
/// Implements human-readable error messages for decoding failures.
impl fmt::Display for DecodeError {
    /// Formats the DecodeError for display purposes.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The formatter to use.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
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
