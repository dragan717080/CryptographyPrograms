use std::collections::HashSet;

fn main() {
    let n: i64 = 295433;
    trial_division(n);
}

/** Most straightforward method, if there is no factor of number up to the square root of n, number is prime
 Performance wise tests such as Miller-Rabin and Fermat's theorem are faster but they aren't 100% guaranteed to give the right result **/
pub fn trial_division(n: i64) -> bool {
    let n_sqrt = (n as f64).sqrt().ceil() as i64;
    let mut is_prime: bool = true;
    for i in 2..=n_sqrt {
        if n % i == 0 {
            is_prime = false;
        }
    }
    //println!("Number is {}prime", if is_prime { "" } else { "not " });
    is_prime
}

pub fn distinct_factors(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();

    for i in 1..=n {
        if n % i == 0 {
            factors.push(i);
        }
    }

    factors
}

pub fn distinct_prime_factors(mut n: i64) -> Vec<i64> {
    let mut factors = Vec::new();

    // Divide by 2 until n is odd
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    // Check odd divisors starting from 3
    let mut divisor = 3;
    while divisor * divisor <= n {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 2; // Move to the next odd number
    }

    // If n is a prime greater than 2
    if n > 2 {
        factors.push(n);
    }

    let distinct_factors_set: HashSet<i64> = factors.into_iter().collect();
    let mut distinct_factors: Vec<_> = distinct_factors_set.into_iter().collect();
    distinct_factors.sort();

    distinct_factors
}
