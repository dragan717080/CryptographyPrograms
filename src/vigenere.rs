mod string_utils;

use string_utils::get_alphabet;

fn main() {
    let message = "MY HIGH CHARMS WORK, AND THESE, MINE ENEMIES, ARE ALL KNIT UPON THEIR DISTRACTIONS. THEY NOW ARE IN MY POWER";
    let key: String = String::from("SHAKESPEARE");
    encrypt(&get_alphabet(), message, &key);
}

// encrypt string using Vigener cypher
fn encrypt(alphabet: &Vec<char>, message: &str, key: &str) -> String {
    let mut encrypted_message = String::new();

    let mut only_letters_index = 0;
    for (_, message_letter) in message.chars().enumerate() {
        if message_letter.is_alphabetic() {
            let letter_index = alphabet
                .iter()
                .position(|&item| item == message_letter)
                .expect("Letter doesn't exist in alphabet");
            // In Vigenere switch is 0 based, switch by A doesn't move the original letter
            //letter_index += 1;

            // Switch by nth letter of key
            let key_index = only_letters_index % key.len();
            let key_letter = key.chars().nth(key_index).expect("");

            let to_move = alphabet.iter().position(|x| x == &key_letter).expect("");

            let new_letter_index = (letter_index + to_move) % alphabet.len();
            let new_letter = alphabet.get(new_letter_index).expect("");

            encrypted_message.push(new_letter.to_owned());
            only_letters_index += 1;
        } else {
            encrypted_message.push(message_letter);
        }
    }

    println!("{}", encrypted_message);
    encrypted_message
}
