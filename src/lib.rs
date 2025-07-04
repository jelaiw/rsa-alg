use std::io::Write;

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
