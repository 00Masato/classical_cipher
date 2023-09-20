// このモジュールでは、大文字の暗号化・復号化を行う Upcase 構造体を定義しています。
pub struct Upcase {
    // 鍵
    key: u8,
    // 暗号化・復号化する文字
    target_char: char,
}

impl Upcase {
    pub fn new(key: u8, target_char: char) -> Self {
        Self { key, target_char }
    }

    // 暗号化
    pub fn encrypt(&self) -> char {
        let mut encrypted_char = self.target_char as u8 + self.key;
        if encrypted_char > 90 {
            encrypted_char = encrypted_char + 26;
        }
        encrypted_char as char
    }

    // 復号化
    pub fn decrypt(&self) -> char {
        let mut decrypted_char = self.target_char as u8 - self.key;
        if decrypted_char < 65 {
            decrypted_char = decrypted_char - 26;
        }
        decrypted_char as char
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encrypt() {
        let encrypted_char = Upcase::new(3, 'A').encrypt();
        assert_eq!(encrypted_char, 'D');
    }

    #[test]
    fn test_decrypt() {
        let decrypted_char = Upcase::new(3, 'D').decrypt();
        assert_eq!(decrypted_char, 'A');
    }
}
