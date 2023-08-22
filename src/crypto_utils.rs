fn main() {
    println!("Utils for cryptography");
}

pub fn find_factors(number: i32) -> Vec<i32> {
    let mut factors = Vec::new();

    for i in 1..=number {
        if number % i == 0 {
            factors.push(i);
        }
    }

    let slice = if factors.len() > 2 { 1..factors.len() - 1} else { 1..factors.len() };
    factors[slice].to_vec()
}

pub fn gcd_euclidean(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn find_modular_inverse(mut n: i32, modulo: i32) -> i32 {
    let mut mod_inverse = 1;
    while (n * mod_inverse) % modulo != 1 {
        mod_inverse += 1;

        // Check for the case where a modular inverse doesn't exist
        if mod_inverse == modulo {
            panic!("Modular inverse does not exist");
        }
    }

    mod_inverse
}
