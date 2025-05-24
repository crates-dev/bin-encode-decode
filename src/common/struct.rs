/// Represents a struct for handling both encode and decode of strings
/// using a custom character set (`charset`). This struct provides methods to
/// encode and decode strings based on the provided `charset`.
///
/// # Fields
/// - `charset`: A reference to the character set used for encoding and decoding.
///   Each character in the `charset` should be unique and ideally contain exactly 64 characters
///   for base64-like encoding.
///
/// # Lifetimes
/// - `'a`: The lifetime `'a` is associated with `charset`, ensuring that the `charset` reference
///   lives at least as long as the `Charset` instance.
#[derive(Debug, Clone)]
pub struct Charset<'a>(pub &'a str);
