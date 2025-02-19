use crate::*;

#[test]
fn test_crypt_decrypt() {
    let mut crypt_decrypt: CryptDecrypt<'_> = CryptDecrypt::new();
    let test_str: &str = "test";
    let mut charset: String = String::new();
    for i in 0..26 {
        let ch: char = ('a' as u8 + i) as char;
        charset.push(ch);
    }
    for i in 0..26 {
        let ch: char = ('A' as u8 + i) as char;
        charset.push(ch);
    }
    for i in 0..10 {
        let ch: char = ('0' as u8 + i) as char;
        charset.push(ch);
    }
    charset.push_str("_=");
    crypt_decrypt.charset(&charset);
    let encode: Result<String, CryptError> = crypt_decrypt.encrypt(test_str);
    let decode: Result<String, DecryptError> = crypt_decrypt.decrypt(&encode.clone().unwrap());
    assert_eq!(decode.unwrap(), test_str);
}
