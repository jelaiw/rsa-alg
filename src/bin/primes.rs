use rsa_alg::get_i64;

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

fn main() {
    loop {
        let m = get_i64("m = ");
        println!("{}", min_divisor(m));
    }
}
