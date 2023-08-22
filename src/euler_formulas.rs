mod primality_tests;

use primality_tests::trial_division;
use primality_tests::distinct_prime_factors;
use primality_tests::distinct_factors;

fn main() {
    //prime_generator();
    totient(120);
    criterion(7, 11);
    phi_function_theorem(120);
}

fn prime_generator() {
    let mut primes = 0;
    for n in 40000..50000 {
        let res = n*n + n + 41;
        let is_prime = trial_division(res);
        if is_prime {
            primes += 1;
        }
    }
    println!("{:.4}% of numbers are prime", (primes as f64)/100.0);
}

// Totient (phi) function (number of integers smaller than n that are relatively prime to n)
fn totient(n_int: i64) -> i64 {
    let mut n = n_int as f64;
    let mut phi = n;
    for prime_factor in distinct_prime_factors(n as i64) {
        phi *= (1.0 - 1.0/(prime_factor as f64));
    }
    println!("Totient of {} is {}", n, phi as i64);

    phi as i64
}

// Checks whether the number n is a quadratic residue for modulo
fn criterion(n: i64, modulus: i64) -> bool {
    // Modulo must be prime
    assert!(trial_division(modulus));

    let exp = (modulus - 1) / 2;
    // Divident^exp mod modulus
    let quotient = (n.pow(exp as u32)) % modulus;
    println!("Quotient: {:?}", quotient);
    // If quotient is 1 means that n is a quadratic residue modulo divisor
    quotient == 1
}

// Phi function over all divisors of n is n itself
fn phi_function_theorem(n: i64) -> bool {
    let sum_phi = distinct_factors(n).iter().fold(0, |acc, &x| acc + totient(x));
    println!("Total totient of all divisors of {} is {}", n, sum_phi);
    sum_phi == n
}
