use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme, PublicKey};
use rand::rngs::OsRng;

fn main() {
    // Generate RSA key pair
    let bits = 2048;
    let private_key = RSAPrivateKey::new(&mut OsRng, bits).expect("Failed to generate private key");

    // Extract public key from private key
    let public_key: RSAPublicKey = private_key.clone().into();

    let message = b"Hello, RSA!";
    let ciphertext_vec = public_key.encrypt(&mut OsRng, PaddingScheme::PKCS1v15Encrypt, message).expect("Encryption failed");
    // The issue with printing ciphertext is that not all ascii codes will correspond to valid characters
    //let ciphertext: String = ciphertext_vec.iter().map(|x| char::from(*x as u8)).collect();
    println!("Ciphertext: {:?}", ciphertext_vec);

    // Decrypt the ciphertext_vec using the private key
    let plaintext_vec = private_key.decrypt(PaddingScheme::PKCS1v15Encrypt, &ciphertext_vec).expect("Decryption failed");
    let plaintext: String = plaintext_vec.iter().map(|x| char::from(*x as u8)).collect();

    println!("Decrypted: {:?}", plaintext);
}
