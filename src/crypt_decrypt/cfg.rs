use crate::*;
use color_output::*;
use std_macro_extensions::*;

#[test]
fn test_crypt_decrypt() {
    let mut crypt_decrypt: CryptDecrypt<'_> = CryptDecrypt::new();
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
    crypt_decrypt.charset(&charset);
    let encode: Result<String, CryptError> = crypt_decrypt.encrypt(test_str);
    let decode: Result<String, DecryptError> = crypt_decrypt.decrypt(&encode.clone().unwrap());
    let encode_str: String = encode.clone().unwrap();
    let decode_str: String = decode.clone().unwrap();
    OutputListBuilder::new()
        .add(
            OutputBuilder::new()
                .endl(true)
                .text(&format!("charset: {}", charset))
                .text_bg_color(ColorType::Use(Color::Blue))
                .text_color(ColorType::Use(Color::Yellow))
                .text_blod(true)
                .build(),
        )
        .add(
            OutputBuilder::new()
                .endl(true)
                .text(&format!("test word: {}", test_str))
                .text_bg_color(ColorType::Use(Color::Yellow))
                .text_color(ColorType::Use(Color::Blue))
                .text_blod(true)
                .build(),
        )
        .add(
            OutputBuilder::new()
                .endl(true)
                .text(&format!("encode_str: {}", encode_str))
                .text_bg_color(ColorType::Use(Color::Cyan))
                .text_color(ColorType::Use(Color::Yellow))
                .text_blod(true)
                .build(),
        )
        .add(
            OutputBuilder::new()
                .endl(true)
                .text(&format!("decode_str: {}", decode_str))
                .text_bg_color(ColorType::Use(Color::Green))
                .text_color(ColorType::Use(Color::Yellow))
                .text_blod(true)
                .build(),
        )
        .run();
    assert_eq!(decode.unwrap(), test_str);
}
