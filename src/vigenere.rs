// ref:https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher

pub struct Vignere {
    // 鍵
    key: String,
    // 平文
    plain_text: String,
}

impl Vignere {
    pub fn new(key: String, plain_text: String) -> Self {
        Self { key, plain_text }
    }

    // 暗号化
    pub fn encrypt(&self) -> String {
        let mut encrypted_text = String::new();
        for i in 0..self.plain_text.len() {
            let plain_char = self.plain_text.chars().nth(i).unwrap();
            let mut key_char: char;
            if i >= self.key.len() {
                key_char = self.key.chars().nth(i - self.key.len()).unwrap();
            } else {
                key_char = self.key.chars().nth(i).unwrap();
            }
            let encrypted_char = self.get_vignere_square()[(plain_char as u8 - 'a' as u8) as usize][(key_char as u8 - 'a' as u8) as usize];
            encrypted_text.push(encrypted_char);
        }
        encrypted_text
    }

    // 復号化
    pub fn decrypt(&self) -> String {
        let mut decrypted_text = String::new();
        for i in 0..self.plain_text.len() {
            let plain_char = self.plain_text.chars().nth(i).unwrap();
            let mut key_char: char;
            if i >= self.key.len() {
                key_char = self.key.chars().nth(i - self.key.len()).unwrap();
            } else {
                key_char = self.key.chars().nth(i).unwrap();
            }
            let decrypted_char = 'a' as u8 + (plain_char as u8 - key_char as u8);
            decrypted_text.push(decrypted_char as char);
        }
        decrypted_text
    }

    // ヴィジュネル方陣を返す
    fn get_vignere_square(&self) -> Vec<Vec<char>> {
        let mut vignere_square = Vec::new();
        let target_map = ('a'..='z').collect::<Vec<char>>();
        for i in 0..26 {
            let mut vignere_row = Vec::new();
            for j in 0..26 {
                if j + i >= 26 {
                    vignere_row.push(target_map[j + i - 26]);
                } else {
                    vignere_row.push(target_map[j + i]);
                }
            }
            vignere_square.push(vignere_row);
        }
        vignere_square
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let vigenere_text = Vignere::new("abc".to_string(), "abc".to_string());
        assert_eq!(vigenere_text.encrypt(), "ace");
    }

    #[test]
    fn test_decrypt() {
        let vigenere_text = Vignere::new("abc".to_string(), "ace".to_string());
        assert_eq!(vigenere_text.decrypt(), "abc");
    }
}
