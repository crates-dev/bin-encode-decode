use super::error::{CryptError, DecryptError};
use super::r#static::CHARSET_LEN;
use super::r#type::CryptDecrypt;
use crate::decrypt::decrypt::decrypt;
use crate::encrypt::encrypt::encrypt;
use std_macro_extensions::*;

impl<'a> Default for CryptDecrypt<'a> {
    fn default() -> Self {
        CryptDecrypt { charset: "" }
    }
}

impl<'a> CryptDecrypt<'a> {
    /// Creates a new instance of `CryptDecrypt` with a default charset.
    pub fn new() -> Self {
        CryptDecrypt::default()
    }

    /// Checks if the `charset` contains `CHARSET_LEN` unique characters.
    ///
    /// # Returns
    /// Returns `true` if `charset` contains `CHARSET_LEN` unique characters, otherwise returns `false`.
    fn judge_charset_safe(&self) -> bool {
        let mut hash_set: HashSet<char> = hash_set!();
        for tmp_char in self.charset.chars() {
            hash_set.insert(tmp_char);
        }
        if hash_set.len() != CHARSET_LEN {
            return false;
        }
        true
    }

    /// Sets the `charset` for the current `CryptDecrypt` instance, if it is not already set.
    ///
    /// # Parameters
    /// - `charset`: A string slice representing the charset to be used.
    ///
    /// # Returns
    /// Returns a mutable reference to `Self` for method chaining.
    pub fn set_charset<'b>(&mut self, charset: &'b str) -> &mut Self
    where
        'b: 'a,
    {
        if self.charset != CryptDecrypt::default().charset {
            return self;
        }
        self.charset = charset;
        self
    }

    /// Encrypts a string based on the current `charset`.
    ///
    /// # Parameters
    /// - `encode_str`: The string slice to be encrypted.
    ///
    /// # Returns
    /// Returns a `Result` containing the encrypted `String` if successful, or a `CryptError` if the charset is invalid.
    pub fn encrypt(&self, encode_str: &str) -> Result<String, CryptError> {
        if !self.judge_charset_safe() {
            return Err(CryptError::CharsetError);
        }
        Ok(encrypt(self.charset, encode_str))
    }

    /// Decrypts a string based on the current `charset`.
    ///
    /// # Parameters
    /// - `decode_str`: The string slice to be decrypted.
    ///
    /// # Returns
    /// Returns a `Result` containing the decrypted `String` if successful, or a `DecryptError` if the charset is invalid.
    pub fn decrypt(&self, decode_str: &str) -> Result<String, DecryptError> {
        if !self.judge_charset_safe() {
            return Err(DecryptError::CharsetError);
        }
        Ok(decrypt(self.charset, decode_str))
    }
}
