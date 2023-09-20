use std::io::{self, Write};
use classical_cipher::caesar::Caesar;
use classical_cipher::vigenere::Vignere;

fn main() {
    loop {
        let mut input = String::new();

        print!("Please enter something(1: encrypt, 2: decrypt): ");
        io::stdout().flush().unwrap();  // プロンプトを表示してから入力を待つ

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "1" => {
                        print!("Please enter cipher type(1: caesar, 2: vigenere): ");
                        io::stdout().flush().unwrap();
                        let mut cipher_type = String::new();
                        io::stdin().read_line(&mut cipher_type).unwrap();
                        match cipher_type.trim() {
                            "1" => {
                                print!("[caesar] Please enter plain text: ");
                                io::stdout().flush().unwrap();
                                let mut plain_text = String::new();
                                // 入力した文字列を plain_text に格納する
                                io::stdin().read_line(&mut plain_text).unwrap();
                                print!("[caesar] Please enter shift: ");
                                io::stdout().flush().unwrap();
                                let mut key = String::new();
                                io::stdin().read_line(&mut key).unwrap();
                                print!("[caesar] text: {}, key: {}\n", plain_text.trim(), key.trim());
                                io::stdout().flush().unwrap();

                                let caesar_text = Caesar::new(key.trim().parse::<u8>().unwrap(), plain_text.trim().to_string());
                                let encrypted_text = caesar_text.encrypt();
                                println!("[caesar] encrypted_text: {}", encrypted_text.unwrap());
                            },
                            "2" => {
                                print!("[vigenere] Please enter plain text: ");
                                io::stdout().flush().unwrap();
                                let mut plain_text = String::new();
                                io::stdin().read_line(&mut plain_text).unwrap();
                                print!("[vigenere] Please enter key: ");
                                io::stdout().flush().unwrap();
                                let mut key = String::new();
                                io::stdin().read_line(&mut key).unwrap();
                                print!("[vigenere] text: {}, key: {}\n", plain_text.trim(), key.trim());
                                io::stdout().flush().unwrap();

                                let vigenere_text = Vignere::new(String::from(key.trim()), plain_text.trim().to_string());
                                let encrypted_text = vigenere_text.encrypt();
                                println!("[vigenere] encrypted_text: {}", encrypted_text);
                            },
                            _ => {
                                println!("run default");
                            }
                        }
                    },
                    "2" => {
                        print!("Please enter cipher type(1: caesar, 2: vigenere): ");
                        io::stdout().flush().unwrap();
                        let mut cipher_type = String::new();
                        io::stdin().read_line(&mut cipher_type).unwrap();
                        match cipher_type.trim() {
                            "1" => {
                                print!("[caesar] Please enter encrypted text: ");
                                io::stdout().flush().unwrap();
                                let mut encrypted_text = String::new();
                                // 入力した文字列を plain_text に格納する
                                io::stdin().read_line(&mut encrypted_text).unwrap();
                                print!("[caesar] Please enter shift: ");
                                io::stdout().flush().unwrap();
                                let mut key = String::new();
                                io::stdin().read_line(&mut key).unwrap();
                                print!("[caesar] encrypted_text: {}, key: {}\n", encrypted_text.trim(), key.trim());
                                io::stdout().flush().unwrap();

                                let caesar_text = Caesar::new(key.trim().parse::<u8>().unwrap(), encrypted_text.trim().to_string());
                                let encrypted_text = caesar_text.decrypt();
                                println!("[caesar] decrypted_text: {}", encrypted_text.unwrap());
                            },
                            "2" => {
                                print!("[vigenere] Please enter encrypted text: ");
                                io::stdout().flush().unwrap();
                                let mut encrypted_text = String::new();
                                io::stdin().read_line(&mut encrypted_text).unwrap();
                                print!("[vigenere] Please enter key: ");
                                io::stdout().flush().unwrap();
                                let mut key = String::new();
                                io::stdin().read_line(&mut key).unwrap();
                                print!("[vigenere] text: {}, key: {}\n", encrypted_text.trim(), key.trim());
                                io::stdout().flush().unwrap();

                                let vigenere_text = Vignere::new(String::from(key.trim()), encrypted_text.trim().to_string());
                                let decrypted_text = vigenere_text.decrypt();
                                println!("[vigenere] decrypted_text: {}", decrypted_text);
                            },
                            _ => {
                                println!("Please enter 1 or 2");
                            }
                        }
                    },
                    _ => {
                        println!("Please enter 1 or 2");
                    }
                }
            },
            Err(e) => {
                println!("Failed to read line: {}", e);
            }
        }
    }
}
