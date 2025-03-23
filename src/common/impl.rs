use crate::*;

impl<'a> Default for Endecode<'a> {
    fn default() -> Self {
        Endecode { charset: "" }
    }
}

impl<'a> Endecode<'a> {
    /// Creates a new instance of `Endecode` with a default charset.
    pub fn new() -> Self {
        Endecode::default()
    }

    /// Checks if the `charset` contains `CHARSET_LEN` unique characters.
    ///
    /// # Returns
    /// Returns `true` if `charset` contains `CHARSET_LEN` unique characters, otherwise returns `false`.
    pub(crate) fn judge_charset_safe(charset: &str) -> bool {
        let mut hash_set: HashSet<char> = HashSet::new();
        for tmp_char in charset.chars() {
            hash_set.insert(tmp_char);
        }
        if hash_set.len() != CHARSET_LEN {
            return false;
        }
        true
    }

    /// Sets the `charset` for the current `Endecode` instance, if it is not already set.
    ///
    /// # Parameters
    /// - `charset`: A string slice representing the charset to be used.
    ///
    /// # Returns
    /// Returns a mutable reference to `Self` for method chaining.
    pub fn charset<'b>(&mut self, charset: &'b str) -> &mut Self
    where
        'b: 'a,
    {
        if self.charset != Endecode::default().charset {
            return self;
        }
        self.charset = charset;
        self
    }

    /// encodes a string based on the current `charset`.
    ///
    /// # Parameters
    /// - `encode_str`: The string slice to be encoded.
    ///
    /// # Returns
    /// Returns a `Result` containing the encoded `String` if successful, or a `EncodeError` if the charset is invalid.
    pub fn encode(&self, encode_str: &str) -> Result<String, EncodeError> {
        encode(self.charset, encode_str)
    }

    /// decodes a string based on the current `charset`.
    ///
    /// # Parameters
    /// - `decode_str`: The string slice to be decoded.
    ///
    /// # Returns
    /// Returns a `Result` containing the decoded `String` if successful, or a `DecodeError` if the charset is invalid.
    pub fn decode(&self, decode_str: &str) -> Result<String, DecodeError> {
        decode(self.charset, decode_str)
    }
}
