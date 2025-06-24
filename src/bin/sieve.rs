use std::collections::HashMap;

fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut map: HashMap<usize, bool> = HashMap::new();
    for n in 2..max {
        if !map.contains_key(&n) { // n is unmarked.
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
    for n in 3..sieve.len() {
        if sieve[n] {
            print!(" {n}");
        }
    }
    println!();
}

fn main() {
    let max = 100;
    let sieve = sieve_of_eratosthenes(max);
    println!("max = {max}");
    print_sieve(&sieve);
}
