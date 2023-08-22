fn main() {
    let plaintext = "DIFFICILIAQUAEPULCHRA";
    
    let encrypted_binary = binary_ascii_encrypt(plaintext);
    println!("Encrypted binary: {}", encrypted_binary);
    let decrypted_binary: String = binary_ascii_decrypt(&encrypted_binary);
    println!("Decrypted binary: {}", decrypted_binary);
}

fn binary_ascii_encrypt(plaintext: &str) -> String {
    let mut encrypted_binary = String::new();

    for c in plaintext.chars() {
        let ascii_code = c as u8;
        let binary_chunk = format!("{:08b}", ascii_code);
        encrypted_binary.push_str(&binary_chunk);
    }

    encrypted_binary
}

fn binary_ascii_decrypt(encrypted_binary: &str) -> String {
    let mut decrypted_text = String::new();

    for chunk in encrypted_binary.chars().collect::<Vec<char>>().chunks(8) {
        let binary_char: String = chunk.into_iter().collect();
        if let Ok(byte) = u8::from_str_radix(&binary_char, 2) {
            //println!("Binary: {} | Decoded Byte: {}", binary_char, byte);
            decrypted_text.push(byte as char);
        } else {
            println!("Failed to decode: {}", binary_char);
        }
    }

    decrypted_text
}
