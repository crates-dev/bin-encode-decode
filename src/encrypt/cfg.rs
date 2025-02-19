use crate::*;

#[test]
fn test_decrypt() {
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
    let encode_res: Result<String, CryptError> = encrypt(&charset, "test");
    assert_eq!(encode_res.unwrap(), "aab0aabLaabZaab0");
}
