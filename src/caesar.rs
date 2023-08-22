mod string_utils;

use string_utils::get_alphabet;
use rand::{self, Rng};

fn main() {
    let message = String::from("AMAT VICTORIA CURAM");
    let alphabet = get_alphabet();
    let encrypted_message: String = encrypt(&alphabet, message);
    decrypt(&alphabet, encrypted_message);
}

fn encrypt(alphabet: &Vec<char>, message: String) -> String {
    //let only_letters: String = message.chars().filter(|x| x.is_alphabetic()).collect();
    let switch_n = rand::thread_rng().gen_range(1..26);
    let mut encrypted_message = String::new();
    for (_, message_letter) in message.chars().enumerate() {
        if !(message_letter.is_alphabetic()) {
            encrypted_message.push(message_letter);
            continue;
        }
        encrypted_message.push(alphabet[(alphabet.iter().position(|x| x == &message_letter).unwrap() + switch_n) % alphabet.len()]);
    }
    println!("Encrypted message: {}", encrypted_message);
    encrypted_message
}

fn decrypt(alphabet: &Vec<char>, encrypted_message: String) {
    // Using the brute force
    for i in 1..26 {
        let mut decrypted_message = String::new();
        for (_, message_letter) in encrypted_message.chars().enumerate() {
            if !(message_letter.is_alphabetic()) {
                decrypted_message.push(message_letter);
                continue;
            }
            decrypted_message.push(alphabet[(alphabet.iter().position(|x| x == &message_letter).unwrap() + i) % alphabet.len()]);
        }
        println!("Decrypted message: {}", decrypted_message);
    }
}
