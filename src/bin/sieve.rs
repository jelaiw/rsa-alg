use std::collections::HashMap;
use rsa_alg::get_i64;

// Build a sieve of Eratosthenes.
fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut map: HashMap<usize, bool> = HashMap::new();
    for n in 2..max {
        if !map.contains_key(&n) { // If n is unmarked, it is prime.
            map.insert(n, true);
        }

        let is_prime = map.get(&n).copied().unwrap();
        if is_prime {
            for i in 2..max {
                let multiple = n * i;
                map.entry(multiple).or_insert(false);
            }
        }
    }

    let mut v = vec![false; max];
    for i in 2..max {
        v[i] = map.get(&i).copied().unwrap();
    }
    v
}

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

// Convert sieve to vector of prime numbers.
fn sieve_to_primes(sieve: &Vec<bool>) -> Vec<i64> {
    let mut v = vec![2];
    // Implement "don't loop over even values" hint from Stephens.
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by
    for i in (3..sieve.len()).step_by(2) {
        if sieve[i] {
            // Note usize to i64 here.
            // Read more at https://stackoverflow.com/a/55769098.
            v.push(i.try_into().unwrap());
        }
    }
    v
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
