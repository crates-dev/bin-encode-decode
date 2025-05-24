use crate::*;

impl Encode {
    /// Encodes a given input string into an encoded format using a specified character set (`charset`).
    /// This function groups bytes in chunks of 3 and maps them into 4-character segments based on `charset`.
    ///
    /// # Parameters
    /// - `charset`: A string representing the character set to use for encoding. Each character
    ///   in `charset` should have a unique position to ensure accurate encoding.
    /// - `encode_str`: The input string to encode. It will be converted to bytes and processed
    ///   in 3-byte chunks.
    ///
    /// # Returns
    /// Returns a `Result` containing the encoded `String` if successful, or a `EncodeError` if the charset is invalid.
    ///
    /// # Example
    /// ```
    /// use bin_encode_decode::*;
    ///
    /// let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=";
    /// let original_str = "test";
    /// let encoded_str = Encode::execute(charset, original_str);
    /// assert_eq!(encoded_str.unwrap(), "aab0aabLaabZaab0");
    /// ```
    pub fn execute(charset: &str, encode_str: &str) -> Result<String, EncodeError> {
        if !Charset::judge_charset_safe(charset) {
            return Err(EncodeError::CharsetError);
        }
        let mut result: String = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        for &byte in encode_str.as_bytes() {
            buffer.extend_from_slice(&[0, 0, byte]);
        }
        for chunk in buffer.chunks(3) {
            let combined: usize =
                ((chunk[0] as usize) << 16) | ((chunk[1] as usize) << 8) | (chunk[2] as usize);
            for i in (0..4).rev() {
                let idx: usize = (combined >> (i * 6)) & 0b111111;
                result.push(charset.chars().nth(idx).unwrap_or_default());
            }
        }
        Ok(result)
    }
}
