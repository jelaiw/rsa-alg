fn gcd(mut a: i64, mut b: i64) -> i64 {
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

fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

fn main() {
    let a = 270;
    let b = 192;
    println!("gcd({a}, {b}) = {}", gcd(a, b));

    let a = 192;
    let b = 270;
    println!("gcd({a}, {b}) = {}", gcd(a, b));

    let a = 12;
    let b = 18;
    println!("gcd({a}, {b}) = {}", gcd(a, b));

    let a = 12;
    let b = 18;
    println!("lcm({a}, {b}) = {}", lcm(a, b));
}
