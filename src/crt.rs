mod crypto_utils;

use crypto_utils::find_modular_inverse;

struct ModularEquation {
    divisor: i64,
    modulo: i64
}

fn main() {
    // Chinese remainder can only be calculated if the divisor and modulo are coprime
    let e1 = ModularEquation { divisor: 3, modulo: 4 };
    let e2 = ModularEquation { divisor: 2, modulo: 5 };

    let clock = e1.modulo * e2.modulo;

    let n1 = e2.modulo;
    let n2 = e1.modulo;

    let n1_inverse = find_modular_inverse(n1, e1.modulo);
    let n2_inverse = find_modular_inverse(n2, e2.modulo);

    let cr = (e1.divisor * n1 * n1_inverse + e2.divisor * n2 * n2_inverse) % clock;

    println!("Chinese remainder is: {} on the clock {}", cr, clock);
}
