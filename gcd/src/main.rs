use std::io::Write;

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

fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    std::io::stdout().flush().unwrap();

    let mut str_value = String::new();
    std::io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    trimmed.parse::<i64>().expect("Error parsing integer")
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
    println!("lcm({a}, {b}) = {}", lcm(a, b));

    loop {
        let a = get_i64("a = ");
        let b = get_i64("b = ");
        println!("gcd({a}, {b}) = {}", gcd(a, b));
        println!("lcm({a}, {b}) = {}", lcm(a, b));
    }
}
