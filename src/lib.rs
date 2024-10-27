#![allow(warnings)]
mod crypt_decrypt;
mod decrypt;
mod encrypt;

pub use crypt_decrypt::{
    error::{CryptError, DecryptError},
    r#type::CryptDecrypt,
};
pub use decrypt::decrypt::decrypt;
pub use encrypt::encrypt::encrypt;
