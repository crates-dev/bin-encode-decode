use crate::*;
use std_macro_extensions::*;

#[test]
fn test_decrypt() {
    let test_str: &str = "test";
    let mut charset: String = string!("");
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
    let encode_res: Result<String, DecryptError> = decrypt(&charset, "aab0aabLaabZaab0");
    assert_eq!(encode_res.unwrap(), "test");
}
