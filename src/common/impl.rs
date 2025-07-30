use crate::*;

/// Provides default implementation for Charset.
///
/// Creates a new Charset instance with empty string as default charset.
///
/// # Returns
///
/// - `Charset<'a>` - New instance with empty charset.
impl<'a> Default for Charset<'a> {
    fn default() -> Self {
        Charset("")
    }
}

impl<'a> Charset<'a> {
    /// Creates a new Charset instance with default charset.
    ///
    /// # Returns
    ///
    /// - `Charset<'a>` - New instance with empty charset.
    pub fn new() -> Self {
        Charset::default()
    }

    /// Validates if charset meets safety requirements.
    ///
    /// # Arguments
    ///
    /// - `&str` - Checks if the charset contains exactly CHARSET_LEN unique characters.
    ///
    /// # Returns
    ///
    /// - `bool` - Validation result.
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

    /// Sets the character set for encoding/decoding operations.
    ///
    /// # Arguments
    ///
    /// - `&'b str` - The character set to use.
    ///
    /// # Returns
    ///
    /// - `&mut Charset<'a>` - Self reference for method chaining.
    pub fn charset<'b>(&mut self, charset: &'b str) -> &mut Self
    where
        'b: 'a,
    {
        if self.0 != Charset::default().0 {
            return self;
        }
        self.0 = charset;
        self
    }

    /// Encodes input string using current charset.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string to encode.
    ///
    /// # Returns
    ///
    /// - `Result<String, EncodeError>` - Encoding result.
    pub fn encode(&self, encode_str: &str) -> Result<String, EncodeError> {
        Encode::execute(self.0, encode_str)
    }

    /// Decodes input string using current charset.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string to decode.
    ///
    /// # Returns
    ///
    /// - `Result<String, DecodeError>` - Decoding result.
    pub fn decode(&self, decode_str: &str) -> Result<String, DecodeError> {
        Decode::execute(self.0, decode_str)
    }
}
