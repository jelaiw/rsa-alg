use rsa_alg::get_i64;

// Credit to Brit Cruise for wall metaphor.
// See https://www.khanacademy.org/computing/computer-science/cryptography/comp-number-theory/a/trial-division.
fn min_divisor(m: i64) -> i64 {
    for d in 2..m {
        if m % d == 0 {
            return d;
        }
        if d * d > m { // sqrt(m) is the "wall".
            return m;
        }
    }
    m
}

#[allow(dead_code)]
fn is_prime(m: i64) -> bool {
    m == min_divisor(m)
}

// Return prime factors of m.
// Algorithm from Kulikov factoring reading.
fn factors(m: i64) -> Vec<i64> {
    let d = min_divisor(m);
    if d == m { // m is prime.
        let v = vec![d];
        v
    }
    else {
        let mut factors = factors(m / d);
        factors.push(d);
        return factors;
    }
}

fn multiply_factors(v: Vec<i64>) -> i64 {
    let mut product = 1;
    for x in v {
        product *= x; // Integer overflow??
    }
    product
}

fn main() {
    loop {
        let m = get_i64("m = ");
        let factors = factors(m);
        println!("Factoring of {m}: {:?}", factors);
        assert_eq!(m, multiply_factors(factors));
    }
}
