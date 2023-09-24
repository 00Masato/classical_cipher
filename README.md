# classical_cipher

**※Do not use this library in a production environment**

This library is about classical cryptographic algorithms.

Currently, the Caesar and Vigenere ciphers are implemented

References:
- Caesar cipher: https://en.wikipedia.org/wiki/Caesar_cipher
- Vigenere cipher: https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher

# Usage
## example
cargo.toml
```
[dependencies]
classical_cipher = { git = "https://github.com/00Masato/classical_cipher" }
```

main.rs
```
use classical_cipher::caesar::Caesar;
use classical_cipher::vigenere::Vigenere;

fn main() {
    let caesar = Caesar::new(3);
    let vigenere = Vigenere::new("key");

    let plain_text = "Hello, world!";
    let encrypted_text = caesar.encrypt(plain_text);
    let decrypted_text = caesar.decrypt(&encrypted_text);

    println!("plain_text: {}", plain_text);
    println!("encrypted_text: {}", encrypted_text);
    println!("decrypted_text: {}", decrypted_text);

    let encrypted_text = vigenere.encrypt(plain_text);
    let decrypted_text = vigenere.decrypt(&encrypted_text);

    println!("plain_text: {}", plain_text);
    println!("encrypted_text: {}", encrypted_text);
    println!("decrypted_text: {}", decrypted_text);
}
```
# License
MIT