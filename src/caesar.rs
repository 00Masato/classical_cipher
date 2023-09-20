// ref: https://en.wikipedia.org/wiki/Caesar_cipher

mod downcase;
mod upcase;

use std::fmt;
use crate::caesar::downcase::Downcase;
use upcase::Upcase;

pub struct Caesar {
    // 鍵
    key: u8,
    // 平文
    plain_text: String,
}

// 暗号化できない文字が含まれている場合に返すエラー
#[derive(Debug, Clone)]
pub struct InvalidCharError {
    message: String,
    invalid_char: char,
}

impl fmt::Display for InvalidCharError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>{
        write!(f, "{}: {}", self.message, self.invalid_char)
    }
}

impl std::error::Error for InvalidCharError {}


impl Caesar {
    pub fn new(key: u8, plain_text: String) -> Self {
        Self { key, plain_text }
    }

    // 暗号化
    // 例：
    // let caesar_text = Caesar::new(3, "ABC".to_string());
    // let encrypted_text = caesar_text.encrypt();
    // println!("{}", encrypted_text); // DEF
    pub fn encrypt(&self) -> Result<String, InvalidCharError> {
        let mut encrypted_text = String::new();
        for char in self.plain_text.chars() {
            let encrypted_char = match char {
                // 小文字
                'a'..='z' => Downcase::new(self.key, char).encrypt(),
                // 大文字
                'A'..='Z' => Upcase::new(self.key, char).encrypt(),
                _ => {
                    return Err(InvalidCharError {
                        message: "暗号化できない文字が含まれています。".to_string(),
                        invalid_char: char,
                    })
                }
            };
            encrypted_text.push(encrypted_char as char);
        }
        Ok(encrypted_text)
    }

    // 復号化
    // 例：
    // let caesar_text = Caesar::new(3, "DEF".to_string());
    // let decrypted_text = caesar_text.decrypt();
    // println!("{}", decrypted_text); // ABC
    pub fn decrypt(&self) -> Result<String, InvalidCharError> {
        let mut decrypted_text = String::new();
        for char in self.plain_text.chars() {
            let decrypted_char = match char {
                // 小文字
                'a'..='z' => Downcase::new(self.key, char).decrypt(),
                // 大文字
                'A'..='Z' => Upcase::new(self.key, char).decrypt(),
                _ => {
                    return Err(InvalidCharError {
                        message: "復号化できない文字が含まれています。".to_string(),
                        invalid_char: char,
                    })
                }
            };
            decrypted_text.push(decrypted_char as char);
        }
        Ok(decrypted_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_only_upcase() {
        let caesar_text = Caesar::new(3, "ABC".to_string());
        let encrypted_text = caesar_text.encrypt().unwrap();
        assert_eq!(encrypted_text, "DEF");
    }

    #[test]
    fn test_encrypt_only_downcase() {
        let caesar_text = Caesar::new(3, "abc".to_string());
        let encrypted_text = caesar_text.encrypt().unwrap();
        assert_eq!(encrypted_text, "def");
    }

    #[test]
    fn test_encrypt_upcase_and_downcase() {
        let caesar_text = Caesar::new(3, "AbC".to_string());
        let encrypted_text = caesar_text.encrypt().unwrap();
        assert_eq!(encrypted_text, "DeF");
    }

    #[test]
    fn test_decrypt_only_upcase() {
        let caesar_text = Caesar::new(3, "DEF".to_string());
        let decrypted_text = caesar_text.decrypt().unwrap();
        assert_eq!(decrypted_text, "ABC");
    }

    #[test]
    fn test_decrypt_only_downcase() {
        let caesar_text = Caesar::new(3, "def".to_string());
        let decrypted_text = caesar_text.decrypt().unwrap();
        assert_eq!(decrypted_text, "abc");
    }

    #[test]
    fn test_decrypt_upcase_and_downcase() {
        let caesar_text = Caesar::new(3, "DeF".to_string());
        let decrypted_text = caesar_text.decrypt().unwrap();
        assert_eq!(decrypted_text, "AbC");
    }
}
