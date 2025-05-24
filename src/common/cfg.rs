use crate::*;

#[test]
fn test_crypt_decode() {
    let mut en_decode: Charset<'_> = Charset::new();
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
    en_decode.charset(&charset);
    let encode: Result<String, EncodeError> = en_decode.encode(test_str);
    let decode: Result<String, DecodeError> = en_decode.decode(&encode.clone().unwrap());
    assert_eq!(decode.unwrap(), test_str);
}
