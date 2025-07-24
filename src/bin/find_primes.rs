use rand::Rng;
use rsa_alg::{get_i64, is_probably_prime};

fn find_prime(min: i64, max: i64, num_tests: u8) -> i64 {
    loop {
        let x = rand::rng().random_range(min..=max);
        if x % 2 != 0 && is_probably_prime(x, num_tests) {
            return x;
        }
    }
}

fn main() {
    const NUM_TESTS: u8 = 20;
    loop {
        let num_digits = get_i64("# digits (max 9): ");
        if num_digits < 1 || num_digits > 9 {
            break;
        }

        let min = 10i64.pow((num_digits - 1) as u32);
        let max = 10 * min;

        let p = find_prime(min, max, NUM_TESTS);
        println!("Probable prime is {}.", p);
    }
}
