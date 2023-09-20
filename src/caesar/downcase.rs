// このモジュールでは、小文字の暗号化・復号化を行う Upcase 構造体を定義しています。
pub(crate) struct Downcase {
    // 鍵
    key: u8,
    // 暗号化・復号化する文字
    target_char: char,
}

impl Downcase {
    pub(crate) fn new(key: u8, target_char: char) -> Self {
        Self { key, target_char }
    }

    // 暗号化
    pub(crate) fn encrypt(&self) -> char {
        let mut encrypted_char = self.target_char as u8 + self.key;
        if encrypted_char > 122 {
            encrypted_char = encrypted_char + 26;
        }
        encrypted_char as char
    }

    // 復号化
    pub(crate) fn decrypt(&self) -> char {
        let mut decrypted_char = self.target_char as u8 - self.key;
        if decrypted_char < 97 {
            decrypted_char = decrypted_char - 26;
        }
        decrypted_char as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let encrypted_char = Downcase::new(3, 'a').encrypt();
        assert_eq!(encrypted_char, 'd');
    }

    #[test]
    fn test_decrypt() {
        let decrypted_char = Downcase::new(3, 'd').decrypt();
        assert_eq!(decrypted_char, 'a');
    }
}
