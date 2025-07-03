use rsa_alg::{get_i64, sieve_of_eratosthenes, sieve_to_primes};

// Print out the primes from the sieve.
fn print_sieve(sieve: &Vec<bool>) {
    print!("2");
    // Implement "don't loop over even values" hint from Stephens.
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by
    for n in (3..sieve.len()).step_by(2) {
        if sieve[n] {
            print!(" {n}");
        }
    }
    println!();
}

// Print the vector of numbers.
fn print_numbers(primes: &Vec<i64>) {
    for prime in primes {
        print!("{prime} ");
    }
    println!();
}

fn main() {
    let max = get_i64("max = ");

    let sieve = sieve_of_eratosthenes(max as usize);
    if max < 1000 {
        print_sieve(&sieve);
    }

    let primes = sieve_to_primes(&sieve);
    if max < 1000 {
        print_numbers(&primes);
    }

    println!("There are {} primes less than {max}.", primes.len());
}
