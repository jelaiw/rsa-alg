use std::collections::HashMap;

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

fn sieve_to_primes(sieve: &Vec<bool>) -> Vec<i64> {
    let mut v = Vec::new();
    for i in 0..sieve.len() {
        if sieve[i] {
            // Note usize to i64 here.
            // Read more at https://stackoverflow.com/a/55769098.
            v.push(i.try_into().unwrap());
        }
    }
    v
}

fn main() {
    let max = 100;
    let sieve = sieve_of_eratosthenes(max);
    println!("max = {max}");
    print_sieve(&sieve);
    let primes = sieve_to_primes(&sieve);
    println!("There are {} primes less than {max}.", primes.len());
    println!("{:?}", primes);
}
