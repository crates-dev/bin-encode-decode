<center>

# bin-encode-decode

[![](https://img.shields.io/crates/v/bin-encode-decode.svg)](https://crates.io/crates/bin-encode-decode)
[![](https://img.shields.io/crates/d/bin-encode-decode.svg)](https://img.shields.io/crates/d/bin-encode-decode.svg)
[![](https://docs.rs/bin-encode-decode/badge.svg)](https://docs.rs/bin-encode-decode)
[![](https://github.com/eastspire/bin-encode-decode/workflows/Rust/badge.svg)](https://github.com/eastspire/bin-encode-decode/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/bin-encode-decode.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/bin-encode-decode/)

A high-performance binary encode and decode library that supports customizable character sets beyond Base64.

## Features

- **Custom Character Sets**: Define your own character set for encoding and decoding, allowing for flexible data representation.
- **High Performance**: Optimized for speed, making it suitable for applications requiring efficient cryptographic operations.
- **Simple API**: Intuitive and easy-to-use interface for both encode and decode processes.
- **Robust Error Handling**: Provides clear and descriptive error messages to facilitate debugging.
- **Extensive Documentation**: Comprehensive guides and examples to help you get started quickly.

## Installation

To install `bin-encode-decode` run cmd:

```sh
cargo add bin-encode-decode
```

## Usage

### encode

#### Use Struct

```rust
use bin_encode_decode::*;
let mut en_decode: Endecode<'_> = Endecode::new();
let test_str: &str = "test";
let mut charset: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=");
en_decode.charset(&charset);
let encode: Result<String, EncodeError> = en_decode.encode(test_str);
```

#### Use Function

```rust
use bin_encode_decode::*;
let test_str: &str = "test";
let mut charset: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=");
let encode: Result<String, EncodeError> = encode(&charset, test_str);
```

### decode

#### Use Struct

```rust
use bin_encode_decode::*;
let mut en_decode: Endecode<'_> = Endecode::new();
let test_str: &str = "aab0aabLaabZaab0";
let mut charset: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=");
en_decode.charset(&charset);
let decode: Result<String, DecodeError> = en_decode.decode(test_str);
```

#### Use Function

```rust
use bin_encode_decode::*;
let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=";
let encoded_str = "aab0aabLaabZaab0";
let decoded_str = decode(charset, encoded_str);
let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=";
let original_str = "test";
let encoded_str = encode(charset, original_str);
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [eastspire <root@ltpp.vip>](mailto:root@ltpp.vip).
