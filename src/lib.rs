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

pub fn fast_exp(num: i64, pow: i64) -> i64 {
    if pow == 0 {
        return 1;
    }
    if pow == 1 {
        return num;
    }
    if pow % 2 == 0 {
        return fast_exp(num * num, pow / 2);
    }
    else {
        return fast_exp(num, pow - 1) * num;
    }
}

pub fn fast_exp_mod(num: i64, pow: i64, modulus: i64) -> i64 {
    if pow == 0 {
        return 1;
    }
    if pow == 1 {
        return num;
    }
    if pow % 2 == 0 {
        return fast_exp_mod(num * num % modulus, pow / 2, modulus);
    }
    else {
        return fast_exp_mod(num, pow - 1, modulus) * num % modulus;
    }
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
