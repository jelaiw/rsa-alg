use rsa_alg::{get_i64, sieve_of_eratosthenes, sieve_to_primes};

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

// Faster if you have ready access to a list of primes.
fn min_divisor_sieve(m: i64, primes: Vec<i64>) -> i64 {
    for d in primes { // Perform trial division only on primes.
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
fn find_factors(m: i64) -> Vec<i64> {
    let d = min_divisor(m);
    if d == m { // m is prime.
        let v = vec![d];
        v
    }
    else {
        let mut factors = find_factors(m / d);
        factors.push(d);
        return factors;
    }
}

fn find_factors_sieve(m: i64, primes: Vec<i64>) -> Vec<i64> {
    let d = min_divisor_sieve(m, primes);
    if d == m { // m is prime.
        let v = vec![d];
        v
    }
    else {
        let mut factors = find_factors(m / d);
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
    let sieve = sieve_of_eratosthenes(10_000_000);
    let primes = sieve_to_primes(&sieve);

    let m = 312680865509917;
    let factors = find_factors_sieve(m, primes);
    println!("Factoring of {m}: {:?}", factors);

    loop {
        let m = get_i64("m = ");
        let factors = find_factors(m);
        println!("Factoring of {m}: {:?}", factors);
        assert_eq!(m, multiply_factors(factors));
    }
}
