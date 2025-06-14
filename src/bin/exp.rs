use rsa_alg::{fast_exp, fast_exp_mod, get_i64};

fn main() {
    let num = 8;
    let pow = 6;
    let modulus = 10;
    println!("{num} ^ {pow} = {}", fast_exp(num, pow));
    println!("{num} ^ {pow} % {modulus} = {}", fast_exp_mod(num, pow, modulus));

    let num = 7;
    let pow = 10;
    let modulus = 101;
    println!("{num} ^ {pow} = {}", fast_exp(num, pow));
    println!("{num} ^ {pow} % {modulus} = {}", fast_exp_mod(num, pow, modulus));

    let num = 9;
    let pow = 13;
    let modulus = 283;
    println!("{num} ^ {pow} = {}", fast_exp(num, pow));
    println!("{num} ^ {pow} % {modulus} = {}", fast_exp_mod(num, pow, modulus));

    let num = 213;
    let pow = 5;
    let modulus = 1000;
    println!("{num} ^ {pow} = {}", fast_exp(num, pow));
    println!("{num} ^ {pow} % {modulus} = {}", fast_exp_mod(num, pow, modulus));

    loop {
        let num = get_i64("num = ");
        let pow = get_i64("pow = ");
        let modulus = get_i64("mod = ");
        println!("{num} ^ {pow} = {}", fast_exp(num, pow));
        println!("{num} ^ {pow} % {modulus} = {}", fast_exp_mod(num, pow, modulus));
    }
}
