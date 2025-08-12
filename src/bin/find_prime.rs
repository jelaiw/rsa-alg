use rsa_alg::{get_i64, find_prime};

fn main() {
    const NUM_TESTS: u8 = 20;

    let prob = 1.0 - 1.0 / 2f64.powi(NUM_TESTS.into());
    println!("Probability of prime = {prob}");

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
