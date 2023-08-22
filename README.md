# Rust Cryptography Examples

This repository contains a collection of cryptographic programs implemented in Rust. Each program demonstrates different cryptographic concepts and algorithms.

## Programs Included

1. RSA Encryption and Decryption
   - Calculate RSA key pairs, encrypt messages, and decrypt ciphertext using the RSA algorithm.

2. Diffie-Hellman Key Exchange
   - Perform the Diffie-Hellman key exchange to securely share a secret key between two parties.

3. Vigenère Cipher
   - Encrypt and decrypt text using the Vigenère cipher, a classic polyalphabetic substitution cipher.

4. Rail Fence Cipher
   - Implement the Rail Fence cipher for simple transposition encryption.

5. Primality Tests
   - Explore various primality testing algorithms, including trial division and the Miller-Rabin test.

6. Euler Prime Theorems
   - Understand Euler's criterion and other related theorems for determining whether a number is a quadratic residue modulo a prime.

## Usage

Each program is located in its own directory with a clear name. To run a specific program, navigate to its directory and follow the instructions provided in the program's source code comments. 

You can build and run the programs using Cargo, Rust's package manager and build tool. Make sure you have Rust installed on your system before proceeding.

Example usage:
```sh
cd rsa
cargo run --bin rsa
