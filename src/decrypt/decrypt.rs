use crate::*;
use std_macro_extensions::*;

/// Decrypts a given encoded string based on a specified charset, using 4-character
/// groups to restore the original bytes. Each character in the `decode_str` string
/// is mapped to an index in `charset` to form the decoded bytes.
///
/// # Parameters
/// - `charset`: A string representing the character set used to encode and decode
///   the data. Each character should have a unique position in `charset`.
/// - `decode_str`: The string to be decoded, which was originally encoded with
///   the provided `charset`.
///
/// # Returns
/// Returns a `Result` containing the decrypted `String` if successful, or a `DecryptError` if the charset is invalid.
///
/// # Example
/// ```
/// use bin_encrypt_decrypt::*;
///
/// let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=";
/// let encoded_str = "aab0aabLaabZaab0";
/// let decoded_str = decrypt(charset, encoded_str);
/// assert_eq!(decoded_str.unwrap(), "test");
/// ```
#[inline]
pub fn decrypt(charset: &str, decode_str: &str) -> Result<String, DecryptError> {
    if !CryptDecrypt::judge_charset_safe(charset) {
        return Err(DecryptError::CharsetError);
    }
    let mut buffer: Vec<u8> = vector!();
    let mut decoded: Vec<u8> = vector!();
    for ch in decode_str.chars() {
        if let Some(idx) = charset.chars().position(|c| c == ch) {
            buffer.push(idx as u8);
        }
        if buffer.len() == 4 {
            let combined: usize = ((buffer[0] as usize) << 18)
                | ((buffer[1] as usize) << 12)
                | ((buffer[2] as usize) << 6)
                | (buffer[3] as usize);
            decoded.push((combined >> 16) as u8);
            decoded.push((combined >> 8) as u8);
            decoded.push(combined as u8);
            buffer.clear();
        }
    }
    let decode_res: String =
        String::from_utf8(decoded.into_iter().filter(|&x| x != 0).collect()).unwrap_or_default();
    Ok(decode_res)
}
