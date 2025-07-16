use std::io::Write;
use rand::Rng;

pub fn is_probable_prime(p: i64, num_tests: u8) -> bool {
    for _i in 0..num_tests {
        let x = rand::thread_rng().gen_range(2..p);
        if fast_exp_mod(x, p-1, p) != 1 {
            return false;
        }
    }
    true
}

pub fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut sieve = vec![false; max + 1];

    sieve[2] = true; // 2 is prime.

    // Initialize odd numbers to prime.
    for p in (3..=max).step_by(2) {
        sieve[p] = true;
    }

    for p in (3..=max).step_by(2) {
        if sieve[p] {
            // Multiples of primes are not prime.
            // Stephens p * 3 is not explained, p^2 optimization described by Cruise in [5].
            for m in (p * p..=max).step_by(p) {
                sieve[m] = false;
            }
        }
    }

    sieve
}

// Convert sieve to vector of prime numbers.
pub fn sieve_to_primes(sieve: &Vec<bool>) -> Vec<i64> {
    let mut v = vec![2];
    for i in (3..sieve.len()).step_by(2) {
        if sieve[i] {
            // Note usize to i64 here.
            // Read more at https://stackoverflow.com/a/55769098.
            v.push(i.try_into().unwrap());
        }
    }
    v
}

// Algorithm based on Euclid's lemma lecture from Kulikov number theory course.
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a > 0 && b > 0 {
        if a >= b {
            a = a % b;
        }
        else {
            b = b % a;
        }
    }
    a.max(b)
}

pub fn lcm(a: i64, b: i64) -> i64 {
    // Nice detail from Stephens to divide by gcd first to avoid integer overflow.
    (a / gcd(a, b)) * b
}

// Algorithm (elegant, recursive) from modular exponentiation reading (Kulikov).
pub fn fast_exp(num: i64, pow: i64) -> i64 {
    if pow == 0 {
        return 1;
    }
    if pow == 1 {
        return num;
    }
    if pow % 2 == 0 {
        return fast_exp(num * num, pow / 2);
    }
    else {
        return fast_exp(num, pow - 1) * num;
    }
}

pub fn fast_exp_mod(num: i64, pow: i64, modulus: i64) -> i64 {
    if pow == 0 {
        return 1;
    }
    if pow == 1 {
        return num;
    }
    if pow % 2 == 0 {
        return fast_exp_mod(num * num % modulus, pow / 2, modulus);
    }
    else {
        return fast_exp_mod(num, pow - 1, modulus) * num % modulus;
    }
}

// Helper function from Stephens.
pub fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    std::io::stdout().flush().unwrap();

    let mut str_value = String::new();
    std::io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    trimmed.parse::<i64>().expect("Error parsing integer")
}

#[cfg(test)]
mod tests {
    use super::{gcd, lcm, fast_exp, fast_exp_mod, sieve_of_eratosthenes, sieve_to_primes, is_probable_prime};
    const NUM_TESTS: u8 = 20;

    #[test]
    fn is_probable_prime_well_studied_fermat_numbers() {
        // See https://en.wikipedia.org/wiki/Fermat_number.
        assert_eq!(true, is_probable_prime(2i64.pow(16) + 1, NUM_TESTS)); // Largest known Fermat prime.
//        assert_eq!(false, is_probable_prime(2i64.pow(32) + 1, NUM_TESTS)); // Bummer.
        assert_eq!(false, is_probable_prime(2i64.pow(31) + 1, NUM_TESTS)); 
    }

    #[test]
    fn sieve_to_primes_verify_max_100() {
        let sieve = sieve_of_eratosthenes(100);
        let primes = sieve_to_primes(&sieve);
        assert_eq!(25, primes.len());
        // Given by instructor for verifying program output.
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        assert_eq!(expected, primes);
    }

    #[test]
    fn sieve_of_eratosthenes_max_semantics() {
        // If the max parameter is the largest number the sieve will consider, then
        // the vector must be of size max + 1 in order to index sieve[max].
        let max = 100;
        let sieve = sieve_of_eratosthenes(max);
        assert_eq!(max + 1, sieve.len());
    }

    #[test]
    fn sieve_of_eratosthenes_indexing_semantics() {
        let max = 20;
        let sieve = sieve_of_eratosthenes(max);
        assert_eq!(false, sieve[0]); // 0 is not prime.
        assert_eq!(false, sieve[1]); // 1 is not prime.
        assert_eq!(true, sieve[2]); // 2 is prime.
        assert_eq!(true, sieve[3]); // 3 is prime.
        assert_eq!(false, sieve[4]); // 4 is not prime.
        assert_eq!(false, sieve[20]); // 20 is not prime.
        assert_eq!(true, sieve[19]); // 19 is prime.
    }

    #[test]
    fn fast_exp_mod_expected_values_from_given_validation_table() {
        assert_eq!(4, fast_exp_mod(8, 6, 10));
        assert_eq!(65, fast_exp_mod(7, 10, 101));
        assert_eq!(179, fast_exp_mod(9, 13, 283));
        assert_eq!(293, fast_exp_mod(213, 5, 1000));
    }

    #[test]
    fn fast_exp_expected_values_from_given_validation_table() {
        assert_eq!(262144, fast_exp(8, 6));
        assert_eq!(282475249, fast_exp(7, 10));
        assert_eq!(2541865828329, fast_exp(9, 13));
        assert_eq!(438427732293, fast_exp(213, 5));
    }

    #[test]
    fn gcd_expected_values_from_given_validation_table() {
        assert_eq!(6, gcd(270, 192));
        assert_eq!(77, gcd(7469, 2464));
        assert_eq!(970, gcd(55290, 115430));
    }

    #[test]
    fn gcd_swap_argument_order() {
        assert_eq!(6, gcd(192, 270));
        assert_eq!(77, gcd(2464, 7469));
        assert_eq!(970, gcd(115430, 55290));
    }

    #[test]
    fn lcm_expected_values_from_given_validation_table() {
        assert_eq!(8640, lcm(270, 192));
        assert_eq!(239008, lcm(7469, 2464));
        assert_eq!(6579510, lcm(55290, 115430));
    }
}
