use rand::{Rng, thread_rng};
use num_bigint::{BigUint, ToBigUint};
use num_traits::cast::ToPrimitive;

fn main() {
    // Step 1: Choose a prime number and a primitive root modulo
    let p: BigUint = 23u64.to_biguint().unwrap();
    let g: BigUint = 5u64.to_biguint().unwrap();

    // Step 2: Select private keys for Alice and Bob
    let private_key_alice: BigUint = compute_private_key(&p).unwrap();
    let private_key_bob: BigUint = compute_private_key(&p).unwrap();

    // Step 3: Compute public keys for Alice and Bob
    let public_key_alice = compute_public_key(&g, &private_key_alice, &p);
    let public_key_bob = compute_public_key(&g, &private_key_bob, &p);

    // Step 4: Compute shared secret
    let shared_secret_alice = compute_shared_secret(&public_key_bob, &private_key_alice, &p);
    let shared_secret_bob = compute_shared_secret(&public_key_alice, &private_key_bob, &p);

    println!("Alice's private key: {}", private_key_alice);
    println!("Bob's private key: {}", private_key_bob);
    println!("Alice's public key: {}", public_key_alice);
    println!("Bob's public key: {}", public_key_bob);
    println!("Shared secret computed by Alice: {}", shared_secret_alice);
    println!("Shared secret computed by Bob: {}", shared_secret_bob);
}

fn compute_private_key(p: &BigUint) -> Option<BigUint> {
    thread_rng().gen_range(1u64..=p.clone().to_u64().unwrap() - 1).to_biguint()
}

fn compute_public_key(g: &BigUint, private_key: &BigUint, p: &BigUint) -> BigUint {
    g.modpow(private_key, p)
}

fn compute_shared_secret(public_key: &BigUint, private_key: &BigUint, p: &BigUint) -> BigUint {
    public_key.modpow(private_key, p)
}
