use rand::Rng;
use rsa_alg::is_probably_prime;

fn find_prime(min: i64, max: i64, num_tests: u8) -> i64 {
    loop {
        let x = rand::rng().random_range(min..=max);
        if is_probably_prime(x, num_tests) {
            return x;
        }
    }
}

fn main() {
    let p = find_prime(2, 999, 20);
    println!("Probable prime is {}.", p);
}
