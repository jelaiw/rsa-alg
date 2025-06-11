use std::io::Write;

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
    (a / gcd(a, b)) * b
}

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
