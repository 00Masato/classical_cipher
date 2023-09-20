use classical_cipher::caesar::Caesar;
use classical_cipher::vigenere::Vignere;

fn main() {
    println!("run in bin");
    let caesar_text = Caesar::new(3, "AbC".to_string());
    let encrypted_text = caesar_text.encrypt();
    println!("{}", encrypted_text.unwrap()); // DeF

    let caesar_text = Caesar::new(3, "DEF".to_string());
    let decrypted_text = caesar_text.decrypt();
    println!("{}", decrypted_text.unwrap()); // ABC

    let vigenere_text = Vignere::new("abc".to_string(), "abc".to_string());
    println!("{}", vigenere_text.encrypt());
}
