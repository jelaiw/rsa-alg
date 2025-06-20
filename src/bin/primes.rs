use rsa_alg::get_i64;

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

fn is_prime(m: i64) -> bool {
    m == min_divisor(m)
}

fn main() {
    loop {
        let m = get_i64("m = ");
        print!("The smallest divisor of {m} is {}", min_divisor(m));
        if is_prime(m) {
            println!(", so it is prime.");
        }
        else {
            println!(".");
        }
    }
}
