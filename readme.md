# bin-encrypt-decrypt

[![](https://img.shields.io/crates/v/bin-encrypt-decrypt.svg)](https://crates.io/crates/bin-encrypt-decrypt)
[![](https://docs.rs/bin-encrypt-decrypt/badge.svg)](https://docs.rs/bin-encrypt-decrypt)
[![](https://img.shields.io/crates/l/bin-encrypt-decrypt.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/bin-encrypt-decrypt/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/bin-encrypt-decrypt/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/BIN-ENCRYPT-DECRYPT/)

A high-performance binary encryption and decryption library that supports customizable character sets beyond Base64.

## Features

- **Custom Character Sets**: Define your own character set for encoding and decoding, allowing for flexible data representation.
- **High Performance**: Optimized for speed, making it suitable for applications requiring efficient cryptographic operations.
- **Simple API**: Intuitive and easy-to-use interface for both encryption and decryption processes.
- **Robust Error Handling**: Provides clear and descriptive error messages to facilitate debugging.
- **Extensive Documentation**: Comprehensive guides and examples to help you get started quickly.

## Installation

To install `bin-encrypt-decrypt` run cmd:

```sh
cargo add bin-encrypt-decrypt
```

## Usage

### Encrypt

#### Use Struct

```rust
use bin_encrypt_decrypt::*;
let mut crypt_decrypt: CryptDecrypt<'_> = CryptDecrypt::new();
let test_str: &str = "test";
let mut charset: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=");
crypt_decrypt.set_charset(&charset);
let encode: Result<String, CryptError> = crypt_decrypt.encrypt(test_str);
```

#### Use Function

```rust
use bin_encrypt_decrypt::*;
let test_str: &str = "test";
let mut charset: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=");
let encode: Result<String, CryptError> = encrypt(&charset, test_str);
```

### Decrypt

#### Use Struct

```rust
use bin_encrypt_decrypt::*;
let mut crypt_decrypt: CryptDecrypt<'_> = CryptDecrypt::new();
let test_str: &str = "aab0aabLaabZaab0";
let mut charset: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=");
crypt_decrypt.set_charset(&charset);
let decode: Result<String, DecryptError> = crypt_decrypt.decrypt(test_str);
```

#### Use Function

```rust
use bin_encrypt_decrypt::*;
let test_str: &str = "aab0aabLaabZaab0";
let mut charset: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=");
let decode: Result<String, DecryptError> = decrypt(&charset, test_str);
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
