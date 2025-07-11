use rsa_alg::{gcd, lcm, get_i64};

fn main() {
    let a = 12;
    let b = 18;
    println!("gcd({a}, {b}) = {}", gcd(a, b));
    println!("lcm({a}, {b}) = {}", lcm(a, b));

    loop {
        let a = get_i64("a = ");
        let b = get_i64("b = ");
        println!("gcd({a}, {b}) = {}", gcd(a, b));
        println!("lcm({a}, {b}) = {}", lcm(a, b));
    }
}
