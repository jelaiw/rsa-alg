use rsa_alg::{get_i64, sieve_of_eratosthenes, sieve_to_primes};
use std::time::Instant;

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
fn min_divisor_sieve(m: i64, primes: &Vec<i64>) -> i64 {
    for d in primes { // Perform trial division only on primes.
        if m % d == 0 {
            return *d;
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

fn find_factors_sieve(m: i64, primes: &Vec<i64>) -> Vec<i64> {
    let d = min_divisor_sieve(m, primes);
    if d == m { // m is prime.
        let v = vec![d];
        v
    }
    else {
        let mut factors = find_factors_sieve(m / d, primes);
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

fn benchmark(m: i64, primes: &Vec<i64>) {
    println!("Number to factor = {m}");

    let start = Instant::now();
    let factors = find_factors(m);
    let duration = start.elapsed();
    assert_eq!(m, multiply_factors(factors));
    println!("find_factors: {:?}", duration);

    let start = Instant::now();
    let factors = find_factors_sieve(m, primes);
    let duration = start.elapsed();
    assert_eq!(m, multiply_factors(factors));
    println!("find_factors_sieve: {:?}", duration);
    println!();
}

fn main() {
    const MAX_PRIME: usize = 10_000_000; // Biggest number sieve will consider.
    let sieve = sieve_of_eratosthenes(MAX_PRIME);
    let primes = sieve_to_primes(&sieve);

    benchmark(312680865509917, &primes);
    benchmark(1819448968910731, &primes);
    benchmark(12345678901234, &primes); // Has small factors, so sieve may not be faster.
    benchmark(6795742697625173, &primes);
    benchmark(64374108854777, &primes); // Big prime, might be slowest.

    loop {
        let m = get_i64("m = ");
        let factors = find_factors_sieve(m, &primes);
        println!("Factoring of {m}: {:?}", factors);
        assert_eq!(m, multiply_factors(factors));
    }
}

#[cfg(test)]
mod tests {
    use super::find_factors;

    #[test]
    fn find_factors_kulikov_examples() {
        assert_eq!(find_factors(7), vec![7]);
        assert_eq!(find_factors(60), vec![5, 3, 2, 2]);
        assert_eq!(find_factors(1001), vec![13, 11, 7]);
        assert_eq!(find_factors(4294967297), vec![6700417, 641]); // 2^32 + 1 from Euler.
//        assert_eq!(find_factors(18446744073709551617), vec![67280421310721, 274177]); // Need bigger integer type for 2^64 + 1.
    }
}
