use crate::*;

#[test]
fn test_decode() {
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
    let encode_res: Result<String, DecodeError> = decode(&charset, "aab0aabLaabZaab0");
    assert_eq!(encode_res.unwrap(), "test");
}
